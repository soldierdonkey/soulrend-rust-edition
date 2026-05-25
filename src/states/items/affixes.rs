pub struct AffixList {
    affix_list: Vec<Affix> //What affixes are in the list.
}
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