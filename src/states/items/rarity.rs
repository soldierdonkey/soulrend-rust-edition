pub enum Rarity {
    // Linear gear progression
    Common, // Basic items with no special properties. These are the most common and least powerful items in the game.
    Uncommon, // Available in early game. 
    Rare, // Highest tier available in the early game.
    Epic, // Available in the mid game.
    Legendary, // Available in the mid game.
    Mythic, // Available in the late game.
    Ancient, // Avalaible in the late game. These items are the first to have unique names.
    Heirloom, // Available in the end game.
    Esoteric, // Available in the end game. These items are the most powerful in the game.

    Exotic, // Special items that don't fit into the normal progression. These can be found in any tier, but are usually very rare and powerful. They often have unique names and properties. They scale based on the other items in the player's inventory, so they can be useful at any stage of the game.
}
pub fn get_next_rarity(current: Rarity) -> Option<Rarity> {
    use Rarity::*;
    match current {
        Common => Some(Uncommon),
        Uncommon => Some(Rare),
        Rare => Some(Epic),
        Epic => Some(Legendary),
        Legendary => Some(Mythic),
        Mythic => Some(Ancient),
        Ancient => Some(Heirloom),
        Heirloom => Some(Esoteric),
        Esoteric => None, // No higher rarity than Esoteric
        Exotic => None, // Exotic doesn't fit into the normal progression
    }
}