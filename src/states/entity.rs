use serde::Deserialize;
use macroquad::prelude::Vec2;
use crate::states::{
    Movement,
    entity::inverse_kinematics::{KinematicsData},
};

pub mod inverse_kinematics;

#[derive(Debug, Clone, Deserialize)]
pub struct Creature {
    pub movement: Movement,
    pub world: usize,
    pub scene: usize,
    pub kinematics: KinematicsData,
}

impl Creature {
    pub fn new() -> Self {
        Self {
            movement: Movement::new(4.0, 10.0, "soulrend:test"),
            world: 0,
            scene: 0,
            kinematics: KinematicsData::default(),
        }
    }

    /// Primary tick called every frame to update position, collisions, animations, and leg IK.
    pub fn update(&mut self, actions: &crate::states::player::controls::Actions, scene_map: &crate::states::environment::scene::SceneMap, dt: f32) {
        // 1. Physics and movement update
        self.movement.update(actions, scene_map, dt);

        // 2. Skeletal spring decay update
        // self.kinematics.update(dt);
    }

    // /// Draws the creature at top_left_pos in pixel coordinates.
    // pub fn draw(&mut self, top_left_pos: Vec2) {
    //     let leg_targets = self.evaluate_leg_targets();
    //     self.kinematics.draw(top_left_pos, Some(leg_targets));
    // }
}