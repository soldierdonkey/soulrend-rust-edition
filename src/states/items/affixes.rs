use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct AffixList {
    affix_list: Vec<Affix> //What affixes are in the list.
}
#[derive(Debug, Deserialize, Clone)]
pub enum Affix {
    // Weapon
    Damage,
    AttackSpeed,
    Omnivamp,
    // Armor
    Armor,
    Health,
    HealthRegen
}