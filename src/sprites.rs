use include_dir::{include_dir, Dir};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::sync::OnceLock;

// Keep these private to the module for safety and encapsulation
static ASSET_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");
static SPRITE_REGISTRY: OnceLock<HashMap<String, Texture2D>> = OnceLock::new();

/// Public function to safely fetch a sprite anywhere in your code
pub fn get(key: &str) -> Option<&Texture2D> {
    SPRITE_REGISTRY.get()?.get(key)
}

pub fn init() {
    let mut registry = HashMap::new();
    
    // Kick off the recursive loading starting at the root directory
    load_dir_recursive(&ASSET_DIR, &mut registry);

    if SPRITE_REGISTRY.set(registry).is_err() {
        eprintln!("Warning: Sprite registry was already initialized.");
    }
}

/// Private helper function to walk down the embedded directory tree
fn load_dir_recursive(dir: &include_dir::Dir, registry: &mut HashMap<String, Texture2D>) {
    // 1. Process all files inside the current folder layer
    for file in dir.files() {
        let path = file.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("png") {
            let mut components = path.components();
            
            // The very first component of the path is always your namespace (e.g., "menu")
            if let Some(namespace_comp) = components.next() {
                let namespace = namespace_comp.as_os_str().to_string_lossy();
                
                // file_stem() automatically extracts just the name "main" out of "menu/main.png"
                if let Some(id_os) = path.file_stem() {
                    let id = id_os.to_string_lossy();
                    let registry_key = format!("{}:{}", namespace, id);
                    
                    println!("Loading sprite -> Key: {}", registry_key);

                    let bytes = file.contents();
                    let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
                    texture.set_filter(FilterMode::Nearest);

                    registry.insert(registry_key, texture);
                }
            }
        }
    }

    // 2. Dive recursively into any subdirectories inside this folder layer
    for subdir in dir.dirs() {
        load_dir_recursive(subdir, registry);
    }
}

pub fn print_all() {
    if let Some(registry) = SPRITE_REGISTRY.get() {
        println!("--- Loaded Sprites ({}) ---", registry.len());
        
        // Collect and sort the keys so they are easy to read alphabetically
        let mut keys: Vec<&String> = registry.keys().collect();
        keys.sort();

        for key in keys {
            println!("  - {}", key);
        }
        println!("--------------------------");
    } else {
        println!("Error: Cannot print sprites. Registry has not been initialized yet.");
    }
}