// Import Inventory
mod inventory;
pub use self::inventory::*;
// Import Attributes
mod attributes;
pub use self::attributes::*;
// Import Stats
mod stats;
pub use self::stats::*;
// Import Position
pub use super::movement::*;
// Import Controls
pub mod controls;
pub use self::controls::*;

pub struct Player {
    pub name: String,
    pub attributes: Attributes,
    pub stats: Stats,
    pub inventory: Inventory,
    pub movement: Movement,
    pub world: usize,
    pub scene: usize,
    pub inputs: Inputs,
}
impl Player {
    pub fn new() -> Self {
        Self {
            name: "Player".to_string(),
            stats: Stats::new(),
            attributes: Attributes::new(),
            inventory: Inventory::new(),
            movement: Movement::new(),
            world: 0,
            scene: 0,
            inputs: Inputs::new(),
        }
    }
}