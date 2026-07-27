// Import Inventory
mod inventory;
use serde::Deserialize;

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
    pub inventory: Inventory,
    pub movement: Movement,
    pub world: usize,
    pub scene: usize,
    pub actions: Actions,
    pub kinematics: KinematicsData
}
impl Player {
    pub fn new() -> Self {
        Self {
            name: "Player".to_string(),
            stats: Stats::new(),
            attributes: Attributes::new(),
            inventory: Inventory::new(),
            movement: Movement::new(4.0, 10.0, "soulrend:test"),
            world: 0,
            scene: 0,
            actions: Actions::new(),
            kinematics: KinematicsData::new("soulrend:test")
        }
    }
}