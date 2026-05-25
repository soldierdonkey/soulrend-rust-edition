mod rarity;
pub use self::rarity::*;
mod weapons;
pub use self::weapons::*;
mod armor;
pub use self::armor::*;
mod affixes;
pub use self::affixes::*;
pub struct Item {
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    // Other item properties (e.g., stats, rarity) can be added here
}
pub enum ItemType {
    Weapon(WeaponData),
    Armor(ArmorData),
    // Add more item types as needed
}