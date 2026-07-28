// Import Inventory
mod inventory;
use serde::Deserialize;

use crate::states::entity::Creature;
use crate::states::entity::inverse_kinematics::KinematicsData;

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

#[derive(Debug, Clone, Deserialize)]
pub struct Player {
    pub name: String,
    pub attributes: Attributes,
    pub stats: Stats,
    pub actions: Actions,
    pub inventory: Inventory,
    pub creature: Creature
}
impl Player {
    pub fn new() -> Self {
        Self {
            name: "Player".to_string(),
            stats: Stats::new(),
            attributes: Attributes::new(),
            inventory: Inventory::new(),
            actions: Actions::new(),
            creature: Creature::new()
        }
    }
}