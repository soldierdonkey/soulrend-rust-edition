use include_dir::{include_dir, Dir};
use macroquad::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::states::{TileType, UiSliceConfig, WidgetElement, WindowType};

// Embed the unified assets directory at compile time
static ASSET_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

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
            keys.sort(); // Keep it alphabetical for easy parsing
            for key in keys {
                println!("  - {}", key);
            }
            println!("==================================================\n");
        } else {
            println!("⚠️ Registry '{}' is currently uninitialized.", registry_name);
        }
    }
}

pub fn dump_all_diagnostics() {
    println!("\n🚀 --- INITIATING ALL REGISTRY DIAGNOSTIC SYSTEM DUMPS --- 🚀\n");

    // Media assets only need key verification so you know the layout paths resolved
    SPRITE_REGISTRY.dump_keys("SPRITES");
    SOUND_REGISTRY.dump_keys("SOUND AUDIO");
    LANG_REGISTRY.dump_keys("LANGUAGE FILES");

    // Structured records benefit from checking properties like alignments or costs
    SOUND_DATA_REGISTRY.dump_detailed("SOUND DATA");
    TILE_DATA_REGISTRY.dump_detailed("TILE BLUEPRINTS");
    ITEM_DATA_REGISTRY.dump_detailed("ITEM DATABASE");
    MOVESET_DATA_REGISTRY.dump_detailed("MOVESET DATABASE");

    // Check your dynamic UI screens to ensure widgets parsed correctly
    GUI_DATA_REGISTRY.dump_detailed("UI SCREENS"); 
}

impl<T: std::fmt::Debug> Registry<T> {
    /// Prints out keys along with their fully formatted internal structural data values
    pub fn dump_detailed(&self, registry_name: &str) {
        if let Some(map) = self.inner.get() {
            println!("=== 🔍 REGISTRY DETAILED STRUCT DUMP: {} ===", registry_name);
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for key in keys {
                if let Some(value) = map.get(key) {
                    // {:#?} prints the struct neatly indented across multiple lines
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
// 1. PLACEHOLDER DATA STRUCTURES (Customize these to fit your game)
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
pub struct ItemRegistryData {
    pub name: String,
    pub attack_power: f32,
    pub durability: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MovesetRegistryData {
    pub animation_speed: f32,
    pub combo_chains: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UiScreenRegistryData {
    pub window: WindowType,
    pub widgets: Vec<WidgetElement>,
}

// =========================================================================
// 2. GLOBAL REGISTRIES
// =========================================================================

// Asset Registries
static SPRITE_REGISTRY: Registry<Texture2D> = Registry::new();
static SOUND_REGISTRY: Registry<Vec<u8>> = Registry::new();
static LANG_REGISTRY: Registry<String> = Registry::new();

// Data Registries
static TILE_DATA_REGISTRY: Registry<TileRegistryData> = Registry::new();
static SOUND_DATA_REGISTRY: Registry<SoundRegistryData> = Registry::new();
static ITEM_DATA_REGISTRY: Registry<ItemRegistryData> = Registry::new();
static MOVESET_DATA_REGISTRY: Registry<MovesetRegistryData> = Registry::new();
static GUI_DATA_REGISTRY: Registry<UiScreenRegistryData> = Registry::new();

// =========================================================================
// 3. PUBLIC ACCESS API (e.g., assets::items::get("namespace:id"))
// =========================================================================
pub mod sprites {
    pub fn get(key: &str) -> Option<&macroquad::prelude::Texture2D> { super::SPRITE_REGISTRY.get(key) }
}
pub mod sounds {
    pub fn get(key: &str) -> Option<&Vec<u8>> { super::SOUND_REGISTRY.get(key) }
}
pub mod langs {
    pub fn get(key: &str) -> Option<&String> { super::LANG_REGISTRY.get(key) }
}
pub mod tiles {
    pub fn get(key: &str) -> Option<&super::TileRegistryData> { super::TILE_DATA_REGISTRY.get(key) }
}
pub mod sound_data {
    pub fn get(key: &str) -> Option<&super::SoundRegistryData> { super::SOUND_DATA_REGISTRY.get(key) }
}
pub mod items {
    pub fn get(key: &str) -> Option<&super::ItemRegistryData> { super::ITEM_DATA_REGISTRY.get(key) }
}
pub mod movesets {
    pub fn get(key: &str) -> Option<&super::MovesetRegistryData> { super::MOVESET_DATA_REGISTRY.get(key) }
}
pub mod uiscreen {
    pub fn get(key: &str) -> Option<&super::UiScreenRegistryData> { super::GUI_DATA_REGISTRY.get(key) }
}

// =========================================================================
// 4. UNIFIED PARSING ENGINE
// =========================================================================
pub fn init() {
    let mut sprites = HashMap::new();
    let mut sounds = HashMap::new();
    let mut langs = HashMap::new();
    
    let mut tile_datas = HashMap::new();
    let mut sound_datas = HashMap::new();
    let mut item_datas = HashMap::new();
    let mut moveset_datas = HashMap::new();
    let mut gui_datas: HashMap<String, UiScreenRegistryData> = HashMap::new();

    load_dir_recursive(
        &ASSET_DIR,
        &mut sprites,
        &mut sounds,
        &mut langs,
        &mut tile_datas,
        &mut sound_datas,
        &mut item_datas,
        &mut moveset_datas,
        &mut gui_datas,
    );

    SPRITE_REGISTRY.init(sprites);
    SOUND_REGISTRY.init(sounds);
    LANG_REGISTRY.init(langs);
    TILE_DATA_REGISTRY.init(tile_datas);
    SOUND_DATA_REGISTRY.init(sound_datas);
    ITEM_DATA_REGISTRY.init(item_datas);
    MOVESET_DATA_REGISTRY.init(moveset_datas);
    GUI_DATA_REGISTRY.init(gui_datas);
}

fn load_dir_recursive(
    dir: &Dir,
    sprites: &mut HashMap<String, Texture2D>,
    sounds: &mut HashMap<String, Vec<u8>>,
    langs: &mut HashMap<String, String>,
    tile_datas: &mut HashMap<String, TileRegistryData>,
    sound_datas: &mut HashMap<String, SoundRegistryData>,
    item_datas: &mut HashMap<String, ItemRegistryData>,
    moveset_datas: &mut HashMap<String, MovesetRegistryData>,
    gui_datas: &mut HashMap<String, UiScreenRegistryData>,
) {
    for file in dir.files() {
        let path = file.path();
        
        let parts: Vec<String> = path
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect();

        // Must follow structure: namespace / asset_type / resources...
        if parts.len() >= 3 {
            let namespace = &parts[0];
            let asset_type = &parts[1];

            // Isolate everything after "namespace/asset_type/"
            let rel_components = &parts[2..];
            if rel_components.is_empty() { continue; }

            // Extract tracking properties safely
            let file_name_with_ext = rel_components.last().unwrap();
            let file_path_util = std::path::Path::new(file_name_with_ext);
            let file_stem = file_path_util.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

            // Determine structural contextual ID (Drop "main" / "data" roots, keep specialized variants)
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

            // Direct Media Asset Routing
            match extension {
                "png" => {
                    let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
                    texture.set_filter(FilterMode::Nearest);
                    sprites.insert(registry_key, texture);
                    continue; // Media matched, skip JSON checks
                }
                "wav" | "ogg" => {
                    sounds.insert(registry_key, bytes.to_vec());
                    continue; 
                }
                "lang" => {
                    if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                        langs.insert(registry_key, text);
                    }
                    continue;
                }
                "json" => {} // Route down to structured data parsing below
                _ => continue, // Unknown extension types safely skipped
            };

            // Structured JSON Data Registry Deserialization Routing
            match asset_type.as_str() {
                "tile" | "tiles" => {
                    match serde_json::from_slice::<TileRegistryData>(bytes) {
                        Ok(data) => { tile_datas.insert(registry_key, data); }
                        Err(e) => eprintln!("Error parsing Tile JSON [{}]: {}", registry_key, e),
                    }
                }
                "sound" | "sounds" => {
                    match serde_json::from_slice::<SoundRegistryData>(bytes) {
                        Ok(data) => { sound_datas.insert(registry_key, data); }
                        Err(e) => eprintln!("Error parsing Sound JSON [{}]: {}", registry_key, e),
                    }
                }
                "item" | "items" => {
                    match serde_json::from_slice::<ItemRegistryData>(bytes) {
                        Ok(data) => { item_datas.insert(registry_key, data); }
                        Err(e) => eprintln!("Error parsing Item JSON [{}]: {}", registry_key, e),
                    }
                }
                "moveset" | "movesets" => {
                    match serde_json::from_slice::<MovesetRegistryData>(bytes) {
                        Ok(data) => { moveset_datas.insert(registry_key, data); }
                        Err(e) => eprintln!("Error parsing Moveset JSON [{}]: {}", registry_key, e),
                    }
                }
                "ui" | "gui" | "uiscreen" | "guiscreen" => {
                    match serde_json::from_slice::<UiScreenRegistryData>(bytes) {
                        Ok(data) => { gui_datas.insert(registry_key, data); }
                        Err(e) => eprintln!("Error parsing Moveset JSON [{}]: {}", registry_key, e),
                    }
                }
                _ => {}
            }
        }
    }

    // Recurse down into directories
    for subdir in dir.dirs() {
        load_dir_recursive(subdir, sprites, sounds, langs, tile_datas, sound_datas, item_datas, moveset_datas, gui_datas);
    }
}

pub fn print_all() {
    println!("--- Unified Database Counters ---");
    println!("  Sprites:   {}", SPRITE_REGISTRY.len());
    println!("  Sounds:    {}", SOUND_REGISTRY.len());
    println!("  Tiles:    {}", TILE_DATA_REGISTRY.len());
    println!("  Items:     {}", ITEM_DATA_REGISTRY.len());
    println!("  Movesets:  {}", MOVESET_DATA_REGISTRY.len());
    println!("  UiScreens:  {}", GUI_DATA_REGISTRY.len());
}