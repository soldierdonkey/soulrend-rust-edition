use include_dir::{include_dir, Dir};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

// Embed the assets directory at compile time
static ASSET_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

/// A generic, thread-safe wrapper around a HashMap for holding global game assets
pub struct Registry<T> {
    inner: OnceLock<HashMap<String, T>>,
}

impl<T> Registry<T> {
    pub const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.inner.get()?.get(key)
    }

    fn init(&self, map: HashMap<String, T>) {
        if self.inner.set(map).is_err() {
            eprintln!("Warning: An asset registry layer was initialized twice.");
        }
    }

    pub fn len(&self) -> usize {
        self.inner.get().map(|m| m.len()).unwrap_or(0)
    }
}

// =========================================================================
// 1. ADD NEW ASSET REGISTRIES HERE
// =========================================================================
static SPRITE_REGISTRY: Registry<Texture2D> = Registry::new();
static LANG_REGISTRY: Registry<String> = Registry::new();
static SOUND_REGISTRY: Registry<Vec<u8>> = Registry::new(); // Stores raw bytes; can be swapped for macroquad::audio::Sound

// =========================================================================
// 3. PUBLIC ACCESS API (asset_type::get("namespace:id"))
// =========================================================================
pub mod sprites {
    pub fn get(key: &str) -> Option<&macroquad::prelude::Texture2D> {
        super::SPRITE_REGISTRY.get(key)
    }
}

pub mod langs {
    pub fn get(key: &str) -> Option<&String> {
        super::LANG_REGISTRY.get(key)
    }
}

pub mod sounds {
    pub fn get(key: &str) -> Option<&Vec<u8>> {
        super::SOUND_REGISTRY.get(key)
    }
}

// =========================================================================
// INITIALIZATION AND PARSING LOGIC
// =========================================================================
pub fn init() {
    let mut sprites = HashMap::new();
    let mut langs = HashMap::new();
    let mut sounds = HashMap::new();

    // Walk the embedded directory structure
    load_dir_recursive(&ASSET_DIR, &mut sprites, &mut langs, &mut sounds);

    // Freeze data into their respective global locks
    SPRITE_REGISTRY.init(sprites);
    LANG_REGISTRY.init(langs);
    SOUND_REGISTRY.init(sounds);
}

/// Private helper to parse paths layout: namespace/asset_type/id
fn load_dir_recursive(
    dir: &Dir,
    sprites: &mut HashMap<String, Texture2D>,
    langs: &mut HashMap<String, String>,
    sounds: &mut HashMap<String, Vec<u8>>,
) {
    for file in dir.files() {
        let path = file.path();
        
        // Convert components to clean String parts for easy checking
        let parts: Vec<String> = path
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect();

        // Valid paths must follow: namespace/asset_type/id...
        if parts.len() >= 3 {
            let namespace = &parts[0];
            let asset_type = &parts[1];

            // Reconstruct the deep ID relative path under the asset type folder (drops namespace & type)
            let mut id_path = PathBuf::new();
            for component in path.components().skip(2) {
                id_path.push(component);
            }

            // Remove file extension to capture just the nested resource ID
            if let Some(id_str) = id_path.with_extension("").to_str() {
                let registry_key = format!("{}:{}", namespace, id_str);
                let bytes = file.contents();

                match asset_type.as_str() {
                    "sprite" | "sprites" => {
                        if path.extension().and_then(|s| s.to_str()) == Some("png") {
                            let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
                            texture.set_filter(FilterMode::Nearest); // Perfect configuration for Soulrend's pixel art style
                            sprites.insert(registry_key, texture);
                        }
                    }
                    "lang" | "langs" => {
                        if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                            langs.insert(registry_key, text);
                        }
                    }
                    "sound" | "sounds" => {
                        sounds.insert(registry_key, bytes.to_vec());
                    }
                    _ => {
                        // Dynamically completely ignores or logs unhandled directories
                    }
                }
            }
        }
    }

    // Recurse into subfolders
    for subdir in dir.dirs() {
        load_dir_recursive(subdir, sprites, langs, sounds);
    }
}

/// Debug helper to inspect tracking arrays across all loaded systems
pub fn print_all() {
    println!("--- Loaded Sprites ({}) ---", SPRITE_REGISTRY.len());
    println!("--- Loaded Langs ({}) ---", LANG_REGISTRY.len());
    println!("--- Loaded Sounds ({}) ---", SOUND_REGISTRY.len());
}