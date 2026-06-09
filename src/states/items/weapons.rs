use serde::Deserialize;

use crate::states::{Affix, AffixList, items::rarity::Rarity};

#[derive(Debug)]
pub struct WeaponData {
    pub affixes: AffixList
}

#[derive(Debug, Deserialize, Clone)]
pub struct WeaponType {
    pub weight_index: f32,
    // Damage and attack speed are calculated from here
    // damage_scale * attack_speed_scale = 1
    // Examples
    // 0.5 = dagger, 0.67 = shortsword, 1 = standard sword. 1.5 = longsword. 2 = hammer/bastard sword.
    pub moveset: String // Lookup for ID
}
pub struct WeaponMoveset {
    pub main_moves: Vec<WeaponMove>,
    pub current_move: usize, // Index of the currently selected move
    pub innate_skill: Option<WeaponMove>, // A special move that can be used regardless of the current moveset, if the weapon has one
    pub end_skill: Option<WeaponMove>, // A powerful finisher move that can only be used when the player has built up enough momentum (e.g., by landing successful hits or dodges). This is meant to encourage aggressive play and reward skillful combat.
}
pub enum WeaponMove {
    Slash(WeaponMoveData),
    Thrust(WeaponMoveData),
    HeavyAttack(WeaponMoveData),
    SpecialMove(WeaponMoveData),
}
pub struct WeaponMoveData {
    pub damage_mult: f32, // Multiplier for the base weapon damage
    pub range: f32, // How far the attack reaches (for hit detection)
    pub attack_speed: f32, // Multiplier for the base weapon attack speed (for animation timing)
}