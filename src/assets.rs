use include_dir::{include_dir, Dir};
use macroquad::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::helper::levenshtein::levenshtein;
use crate::states::widget::WidgetList;
use crate::states::{TileType, UiSliceConfig, WindowType};
use crate::states::items::ItemRegistryData;

pub mod sprite_modularity;
use crate::assets::sprite_modularity::*;

// Embed the unified assets directory at compile time
static ASSET_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

// Thread-safe global storage for the single configured font
static FONT_REGISTRY: OnceLock<Font> = OnceLock::new();

pub mod fonts {
    use super::*; 
    
    /// Returns a reference to the global font if it was successfully loaded.
    pub fn get() -> Option<&'static Font> {
        FONT_REGISTRY.get()
    }
}

/// Generic, thread-safe global registry wrapper
pub struct Registry<T> {
    inner: OnceLock<HashMap<String, T>>,
}

impl<T> Registry<T> {
    pub const fn new() -> Self {
        Self { inner: OnceLock::new() }
    }
    pub fn get(&self, key: &str) -> Option<&T> {
        self.inner.get()?.get(key)
    }
    fn init(&self, map: HashMap<String, T>) {
        if self.inner.set(map).is_err() {
            eprintln!("Warning: Registry layer was initialized twice.");
        }
    }
    pub fn len(&self) -> usize {
        self.inner.get().map(|m| m.len()).unwrap_or(0)
    }
    pub fn dump_keys(&self, registry_name: &str) {
        if let Some(map) = self.inner.get() {
            println!("=== 📦 REGISTRY KEY DUMP: {} ({} items) ===", registry_name, map.len());
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort(); 
            for key in keys {
                println!("  - {}", key);
            }
            println!("==================================================\n");
        } else {
            println!("⚠️ Registry '{}' is currently uninitialized.", registry_name);
        }
    }
    pub fn find_similar(&self, key: &str, limit: usize) -> Vec<String> {
        let Some(map) = self.inner.get() else { return vec![]; };
        let mut distances: Vec<(&String, usize)> = map.keys()
            .map(|k| (k, levenshtein(k, key)))
            .collect();
        
        distances.sort_by_key(|&(_, dist)| dist);
        distances.into_iter().take(limit).map(|(k, _)| k.clone()).collect()
    }
}

impl<T: std::fmt::Debug> Registry<T> {
    pub fn dump_detailed(&self, registry_name: &str) {
        if let Some(map) = self.inner.get() {
            println!("=== 🔍 REGISTRY DETAILED STRUCT DUMP: {} ===", registry_name);
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for key in keys {
                if let Some(value) = map.get(key) {
                    println!("  ▶ [{}] => {:#?}", key, value);
                }
            }
            println!("==================================================\n");
        } else {
            println!("⚠️ Registry '{}' is currently uninitialized.", registry_name);
        }
    }
}

// =========================================================================
// 1. DATA STRUCTURES
// =========================================================================
#[derive(Debug, Deserialize, Clone)]
pub struct TileRegistryData {
    pub name: String,
    pub tile_type: TileType,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SoundRegistryData {
    pub volume: f32,
    pub pitch_variance: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MovesetRegistryData {
    pub animation_speed: f32,
    pub combo_chains: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UiScreenRegistryData {
    pub window: WindowType,
    pub widgets: WidgetList,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThreePatchRegistryData {
    pub border_size: f32,
    pub padding: f32
}

// =========================================================================
// 2. UNIFIED REGISTRY ENGINE MACRO
// =========================================================================
macro_rules! declare_registries {
    (
        custom_assets: {
            $(
                $asset_name:ident : $asset_type:ty => {
                    static: $asset_static:ident,
                    mod: $asset_mod:ident,
                    label: $asset_label:literal,
                    extensions: [$($ext:literal),+],
                    dump: $asset_dump_method:ident ($asset_dump_name:literal),
                    parse: |$bytes_ident:ident, $path_ident:ident| $parse_expr:expr
                }
            ),* $(,)?
        },
        json_data: {
            $(
                $data_name:ident : $data_type:ty => {
                    static: $data_static:ident,
                    mod: $data_mod:ident,
                    label: $data_label:literal,
                    folders: [$($folder:literal),+],
                    dump: $data_dump_method:ident ($data_dump_name:literal),
                }
            ),* $(,)?
        }
    ) => {
        // Generate global underlying static records
        $( static $asset_static: Registry<$asset_type> = Registry::new(); )*
        $( static $data_static: Registry<$data_type> = Registry::new(); )*

        // Generate public APIs dynamically (e.g. sprites::get)
        $(
            pub mod $asset_mod {
                use super::*; 
                pub fn get(key: &str) -> Option<&$asset_type> {
                    $asset_static.get(key)
                }
                pub fn find_similar(key: &str) -> Vec<String> { $asset_static.find_similar(key, 3) }
            }
        )*
        $(
            pub mod $data_mod {
                use super::*; 
                pub fn get(key: &str) -> Option<&$data_type> {
                    $data_static.get(key)
                }
                pub fn find_similar(key: &str) -> Vec<String> { $data_static.find_similar(key, 3) }
            }
        )*

        // An internal context structure to pass maps cleanly down the recursive loop
        struct RegistryContext {
            $( $asset_name: HashMap<String, $asset_type>, )*
            $( $data_name: HashMap<String, $data_type>, )*
        }

        pub fn init() {
            let mut ctx = RegistryContext {
                $( $asset_name: HashMap::new(), )*
                $( $data_name: HashMap::new(), )*
            };

            load_dir_recursive(&ASSET_DIR, &mut ctx);

            // =============================================
            // THIS PART IS HARDCODED, LOADS MODULAR SPRITES
            // =============================================

            // Integrated colors in palettes
            
            for (palette_id, palette_data) in &mut ctx.palette {
                for (_color_id, color_data) in palette_data.colors.iter_mut() {
                    let new = color_data.replace("self", palette_id);
                    *color_data = new;
                }
                if let Some(integrated_hexcolors) = palette_data.integrated_colors.take() {
                    for (hexcolor_id, hexcolor_data) in &integrated_hexcolors {
                        ctx.hexcolor.insert(
                            //namespace:id/color
                            format!{"{}/{}", palette_id, hexcolor_id},
                            HexColor { hex: *hexcolor_data }
                        );
                    }
                }
            }

            // Integrated sprites in items

            for (item_id, item_data) in &mut ctx.items {
                if let Some(integrated_sprite) = item_data.integrated_sprite.take() {
                    ctx.modularsprite.insert(
                        item_id.clone(),
                        integrated_sprite
                    );
                }
            }

            // modular sprite loading

            for (modular_sprite_id, modular_sprite) in &ctx.modularsprite {
                ctx.sprites.insert(
                    modular_sprite_id.clone(),
                    modular_sprite.init()
                );
            }

            // Load the single global font file from assets/config/font.ttf
            if let Some(font_file) = ASSET_DIR.get_file("config/font.ttf") {
                match load_ttf_font_from_bytes(font_file.contents()) {
                    Ok(font) => {
                        if FONT_REGISTRY.set(font).is_err() {
                            eprintln!("Warning: Font registry layer was initialized twice.");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: Failed to parse 'config/font.ttf': {:?}", e);
                    }
                }
            } else {
                eprintln!("Warning: Font file 'config/font.ttf' was not found inside the assets directory.");
            }
            // =============================================
            //              END OF HARDCODING
            // =============================================

            $( $asset_static.init(ctx.$asset_name); )*
            $( $data_static.init(ctx.$data_name); )*
        }

        fn load_dir_recursive(dir: &Dir, ctx: &mut RegistryContext) {
            for file in dir.files() {
                let path = file.path();
                let parts: Vec<String> = path
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().into_owned())
                    .collect();

                if parts.len() >= 3 {
                    let namespace = &parts[0];
                    let asset_type = &parts[1];
                    let rel_components = &parts[2..];
                    if rel_components.is_empty() { continue; }

                    let file_name_with_ext = rel_components.last().unwrap();
                    let file_path_util = std::path::Path::new(file_name_with_ext);
                    let file_stem = file_path_util.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

                    let id_str = if file_stem == "main" || file_stem == "data" {
                        let parent_components = &rel_components[0..rel_components.len() - 1];
                        parent_components.join("/")
                    } else {
                        let mut modified_components = rel_components.to_vec();
                        modified_components[rel_components.len() - 1] = file_stem.to_string();
                        modified_components.join("/")
                    };

                    let registry_key = format!("{}:{}", namespace, id_str);
                    let bytes = file.contents();

                    // Media Asset Routing Engine
                    $(
                        if $( extension == $ext )||* {
                            let parse_closure = |$bytes_ident: &[u8], $path_ident: &std::path::Path| -> Option<$asset_type> {
                                $parse_expr
                            };
                            if let Some(parsed_val) = parse_closure(bytes, path) {
                                ctx.$asset_name.insert(registry_key, parsed_val);
                            }
                            continue;
                        }
                    )*

                    if extension != "json" { continue; }

                    // Structured JSON Data Deserialization Engine
                    match asset_type.as_str() {
                        $(
                            $( $folder )|* => {
                                match serde_json::from_slice::<$data_type>(bytes) {
                                    Ok(data) => { ctx.$data_name.insert(registry_key, data); }
                                    Err(e) => eprintln!("Error parsing JSON [{}]: {}", registry_key, e),
                                }
                            }
                        )*
                        _ => {}
                    }
                }
            }

            for subdir in dir.dirs() {
                load_dir_recursive(subdir, ctx);
            }
        }

        pub fn dump_all_diagnostics() {
            println!("\n🚀 --- INITIATING ALL REGISTRY DIAGNOSTIC SYSTEM DUMPS --- 🚀\n");
            $( $asset_static.$asset_dump_method($asset_dump_name); )*
            $( $data_static.$data_dump_method($data_dump_name); )*
        }

        pub fn print_all() {
            println!("--- Unified Database Counters ---");
            $( println!("  {:<24} {}", format!("{}:", $asset_label), $asset_static.len()); )*
            $( println!("  {:<24} {}", format!("{}:", $data_label), $data_static.len()); )*
            println!("  Font Global:             {}", if FONT_REGISTRY.get().is_some() { "✅" } else { "❌" });
        }
    };
}

// =========================================================================
// 3. DECLARE THE REGISTRIES (Declarative Configuration)
// =========================================================================
declare_registries! {
    custom_assets: {
        sprites: Texture2D => {
            static: SPRITE_REGISTRY,
            mod: sprites,
            label: "Sprites",
            extensions: ["png"],
            dump: dump_keys("SPRITES"),
            parse: |bytes, _path| {
                let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
                texture.set_filter(FilterMode::Nearest);
                Some(texture)
            }
        },
        sounds: Vec<u8> => {
            static: SOUND_REGISTRY,
            mod: sounds,
            label: "Sounds",
            extensions: ["wav", "ogg"],
            dump: dump_keys("SOUND AUDIO"),
            parse: |bytes, _path| Some(bytes.to_vec())
        },
        langs: String => {
            static: LANG_REGISTRY,
            mod: langs,
            label: "Languages",
            extensions: ["lang"],
            dump: dump_keys("LANGUAGE FILES"),
            parse: |bytes, _path| String::from_utf8(bytes.to_vec()).ok()
        },
    },

    json_data: {
        tiles: TileRegistryData => {
            static: TILE_DATA_REGISTRY,
            mod: tiles,
            label: "Tiles",
            folders: ["tile", "tiles"],
            dump: dump_detailed("TILE BLUEPRINTS"),
        },
        sound_data: SoundRegistryData => {
            static: SOUND_DATA_REGISTRY,
            mod: sound_data,
            label: "Sound Data",
            folders: ["sound", "sounds"],
            dump: dump_detailed("SOUND DATA"),
        },
        items: ItemRegistryData => {
            static: ITEM_DATA_REGISTRY,
            mod: items,
            label: "Items",
            folders: ["item", "items"],
            dump: dump_detailed("ITEM DATABASE"),
        },
        movesets: MovesetRegistryData => {
            static: MOVESET_DATA_REGISTRY,
            mod: movesets,
            label: "Movesets",
            folders: ["moveset", "movesets"],
            dump: dump_detailed("MOVESET DATABASE"),
        },
        uiscreen: UiScreenRegistryData => {
            static: GUI_DATA_REGISTRY,
            mod: uiscreen,
            label: "UiScreens",
            folders: ["ui", "gui", "uiscreen", "guiscreen"],
            dump: dump_detailed("UI SCREENS"),
        },
        threepatch: ThreePatchRegistryData => {
            static: THREE_PATCH_REGISTRY,
            mod: threepatch,
            label: "ThreePatches",
            folders: ["threepatch", "three_patch", "threepatches", "three_patches", "window", "windows"],
            dump: dump_detailed("THREE PATCH WINDOWS"),
        },
        hexcolor: HexColor => {
            static: HEX_COLOR_REGISTRY,
            mod: hexcolor,
            label: "HexColor",
            folders: ["color", "colors", "hexcolor", "hexcolors", "hex"],
            dump: dump_detailed("HEX COLOR DATABASE"),
        },
        palette: Palette => {
            static: PALETTE_REGISTRY,
            mod: palette,
            label: "Palette",
            folders: ["palette", "palettes"],
            dump: dump_detailed("PALETTE DATABASE"),
        },
        modularspritelayer: ModularSpriteLayer => {
            static: MODULAR_SPRITE_LAYER_REGISTRY,
            mod: modularspritelayer,
            label: "Modular Sprite Layer",
            folders: ["modular_sprite_layer", "modular_sprite_layers", "sprite_layer", "sprite_layers"],
            dump: dump_detailed("MODULAR SPRITE LAYER DATABASE"),
        },
        modularsprite: ModularSprite => {
            static: MODULAR_SPRITE_REGISTRY,
            mod: modularsprite,
            label: "Modular Sprite",
            folders: ["modular_sprite", "modular_sprites", "mod_sprites", "mod_sprite"],
            dump: dump_detailed("MODULAR SPRITE DATABASE"),
        },
    }
}