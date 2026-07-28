use serde::Deserialize;

use crate::states::{Movement, entity::inverse_kinematics::KinematicsData};

pub mod inverse_kinematics;

#[derive(Debug, Clone, Deserialize)]
pub struct Creature {
    pub movement: Movement,
    pub world: usize,
    pub scene: usize,
    pub kinematics: KinematicsData
}

impl Creature {
    pub fn new() -> Self {
        Self {
            movement: Movement::new(4.0, 10.0, "soulrend:test"),
            world: 0,
            scene: 0,
            kinematics: KinematicsData::new("soulrend:test")
        }
    }
}