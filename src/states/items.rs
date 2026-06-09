mod rarity;
use serde::Deserialize;

use crate::assets;

pub use self::rarity::*;
mod weapons;
pub use self::weapons::*;
mod armor;
pub use self::armor::*;
mod affixes;
pub use self::affixes::*;

#[derive(Debug)]
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
    pub item_type: ItemType,
    pub rarity: Rarity
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

#[derive(Debug)]
pub enum ItemData {
    Weapon(WeaponData),
    Armor,
}