use crate::states::player::*;
use crate::states::environment::tile::*;
use crate::states::environment::scene::*;

impl Player {
    pub fn move_player(&mut self, scene: &SceneMap, delta_time: f32) {
        self.creature.movement.update(&self.actions, scene, delta_time);
    }
}