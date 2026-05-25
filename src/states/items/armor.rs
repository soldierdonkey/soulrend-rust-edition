use serde::Deserialize;

use crate::states::items::rarity::Rarity;
pub struct ArmorData {
    pub defense: f32,
    pub weight: f32,
    pub slot: ArmorSlot,
    pub rarity: Rarity,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum ArmorSlot {
    Head,
    Chest,
    Legs,
    Feet,
}