use serde::Deserialize;

use crate::states::{AffixList, items::rarity::Rarity};


#[derive(Debug, Deserialize, Clone)]
pub struct ArmorType {
    pub defense: f32,
    pub weight: f32,
    pub slot: ArmorSlot,
    pub rarity: Rarity,
    pub affixes: AffixList
}
impl ArmorType {
    pub fn get_index(&self) -> usize {
        slot_to_index(&self.slot)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum ArmorSlot {
    Head,
    Chest,
    Legs,
    Feet,
}
// converts armor slot to index
pub fn slot_to_index(slot: &ArmorSlot) -> usize {
    match slot {
        ArmorSlot::Head => 0,
        ArmorSlot::Chest => 1,
        ArmorSlot::Legs => 2,
        ArmorSlot::Feet => 3
    }
}