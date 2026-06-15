use macroquad::prelude::*;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use crate::assets;

use crate::helper::hex_to_color::hex_to_color;
use crate::helper::levenshtein::levenshtein;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct HexColor {
    pub hex: u32
}
impl HexColor {
    pub fn to_color(&self) -> Color {
        hex_to_color(self.hex)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Palette {
    // Name of Palette, for display (e.g. Crimson or Wulfrum, not soulrend:crimson or soulrend:wulfrum)
    pub name: String,
    // Map scheme names (e.g., "highlight") to asset IDs (e.g., "ui:highlight_color")
    pub colors: BTreeMap<String, String>,
    // Store thresholds separately to keep the struct clean
    pub thresholds: Vec<(f32, String)>,
    // Integrated Colors, Optional
    pub integrated_colors: Option<HashMap<String, u32>>,
    // Background color
    pub background: Option<String>
}

impl Palette {
    /// Retrieves a color from the registry by its scheme name
    pub fn get_color(&self, scheme: &str) -> HexColor {
        let color_id = self.colors.get(scheme).unwrap_or_else(|| crate::global_panic!(palette scheme => self));

        match assets::hexcolor::get(color_id) {
            Some(hex) => *hex,
            None => crate::global_panic!(asset hexcolor color_id),
        }
    }

    /// Returns the color corresponding to the brightness, 
    /// defaulting to the lowest threshold if none match.
    pub fn brightness_to_color(&self, brightness: f32) -> HexColor {
        // Iterate through thresholds (assumed sorted descending)
        for (threshold, scheme) in &self.thresholds {
            if brightness > *threshold {
                return self.get_color(scheme);
            }
        }
        // Fallback for the lowest/shadow range
        self.get_color(&self.thresholds.last().unwrap().1)
    }
    pub fn brightness_to_level(&self, brightness: f32) -> String {
        // Iterate through thresholds (assumed sorted descending)
        for (threshold, scheme) in &self.thresholds {
            if brightness > *threshold {
                return scheme.clone();
            }
        }
        // Fallback for the lowest/shadow range
        self.thresholds.last().unwrap().1.clone()
    }

    // Palette has a lookup. This is for error management.
    pub fn find_similar(&self, key: &str, limit: usize) -> Vec<String> {
        let map = self.colors.keys();
        let mut distances: Vec<(&String, usize)> = map
            .map(|k| (k, levenshtein(k, key)))
            .collect();
        
        distances.sort_by_key(|&(_, dist)| dist);
        distances.into_iter().take(limit).map(|(k, _)| k.clone()).collect()
    }
}

// 2. Define a sprite layer
#[derive(Debug, Deserialize, Clone)]
pub struct ModularSpriteLayer {
    pub base: String, // Greyscale Sprite ID
    pub palette: String // Palette ID
}

// 3. Define a modular sprite
#[derive(Debug, Deserialize, Clone)]
pub struct ModularSprite {
    pub layers: Vec<ModularSpriteLayer>, // ModularSpriteLayers, not IDs.
}

// Generate a texture from a grayscale image template
impl ModularSprite {
    pub fn init(&self) -> Texture2D {
        // Check if more than one layer exist
        if self.layers.len() == 0 {
            crate::global_panic!(empty "ModularSprite", self)
        }
        let layer_list: Vec<(&Texture2D, &Palette)> = self.layers.iter().map(
            |layer|
            (
                assets::sprites::get(&layer.base).unwrap_or_else(|| crate::global_panic!(asset sprites &layer.base)),
                assets::palette::get(&layer.palette).unwrap_or_else(|| crate::global_panic!(asset palette &layer.palette))
            )
        ).collect();
        // Check if all dimensions are correct
        let initial_dimesions = layer_list[0].0.size();
        // Panic if dimensions are mismatched
        layer_list.iter().for_each(|layer| if layer.0.size() != initial_dimesions {crate::global_panic!(mismatch "ModularSpriteLayer Base", initial_dimesions => layer.0.size())});
        // Define starting image
        let mut baked_image = Image::gen_image_color(initial_dimesions.x as u16, initial_dimesions.y as u16, BLANK);

        layer_list.iter().for_each(|layer| {
            let image = layer.0.get_texture_data();
            for y in 0..image.height as u32 {
                for x in 0..image.width as u32 {
                    let pixel = image.get_pixel(x, y);
                    
                    // Skip transparent pixels
                    if pixel.a == 0.0 { continue; }

                    // Map grayscale values to palette choices
                    // Assuming templates use red channel (or brightness) to determine depth:
                    let brightness = pixel.r; 

                    baked_image.set_pixel(x, y, layer.1.brightness_to_color(brightness).to_color());
                }
            }
            }
        );

        // Convert the computed image into a GPU texture ready for your render pipelines
        Texture2D::from_image(&baked_image)
    }
}