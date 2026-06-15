mod rarity;
use macroquad::math::Rect;
use serde::Deserialize;

use crate::assets;
use crate::assets::sprite_modularity::ModularSprite;
use crate::helper;
use crate::helper::draw_rich_text;
use crate::runtime::Runtime;
use crate::runtime::get_mouse_position;

pub use self::rarity::*;
mod weapons;
pub use self::weapons::*;
mod armor;
pub use self::armor::*;
mod affixes;
pub use self::affixes::*;

#[derive(Debug, Deserialize)]
pub struct Item {
    pub id: String,
    pub item_data: ItemData,
    // Other item properties (e.g., stats, rarity) can be added here
}
impl Item {
    pub fn new(id: String) -> Self {
        Self {
            id: id,
            item_data: ItemData::Armor
        }
    }
    pub fn get_sprite(&self) -> String {
        format!("{}/inventory", self.id) // TODO Check out if this is too complex
    }
}

// ===========================
//       ITEM CONSTRUCTOR
// ===========================

#[derive(Debug, Deserialize, Clone)]
pub struct ItemRegistryData {
    pub name: String,
    pub sprite: String,
    pub integrated_sprite: Option<ModularSprite>,
    pub item_type: ItemType,
    pub rarity: Rarity,
    pub palette: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum ItemType {
    Weapon(WeaponType),
    Armor(ArmorType),
    Material
    // Add more item types as needed
}

// ===========================
//         PER-ITEM DATA
// ===========================

#[derive(Debug, Deserialize)]
pub enum ItemData {
    Weapon(WeaponData),
    Armor,
}

// ===========================
//      TOOLTIP GENERATOR
// ===========================

impl Runtime {
    // Generate Tooltip is on Runtime because who the fuck knows what your gamestate is. Depends both on inputs and player data.
    pub fn generate_tooltip(&self, item: &Item) -> Vec<String> {
        let item_data = assets::items::get(&item.id).unwrap_or_else(|| crate::global_panic!(data items &item.id));
        let palette = assets::palette::get(&item_data.palette).unwrap_or_else(|| crate::global_panic!(data items &item_data.palette));
        let mut output: Vec<String> = Vec::new();
        // TITLE
        output.push(format!("§centered§§color:{}§{}", palette.brightness_to_level(0.55), item_data.name)); // Get main color
        output.push(format!("§hr:{}§", palette.brightness_to_level(0.0))); // Slight overhead here, pretty much just want lowest level (might change in the future)
        // Data
        match &item_data.item_type {
            ItemType::Weapon(weapon_type) => {
                output.push(format!("Weight: {}", weapon_type.weight_index));
            },
            ItemType::Armor(amror_type) => {
                
            },
            ItemType::Material => {
                
            }
        }
        // Return
        if crate::debug::debug_frame() {
            println!("▶ Dumping item tooltip for item: [{}] =>\n {:#?}", item.id, output)
        }
        output
    }
    pub fn draw_tooltip(&self, item: &Item) {
        if let Some(item_entry) = assets::items::get(&item.id) {
            let mouse_position = get_mouse_position();
            let palette = assets::palette::get(&item_entry.palette).unwrap_or_else(|| crate::global_panic!(data palette &item_entry.palette));
            draw_rich_text(
                &self.generate_tooltip(&item), 
                mouse_position.x,
                mouse_position.y,
                crate::LOGICAL_WIDTH / 2.0,
               &palette.background,
                palette
            );
        }
    }
}