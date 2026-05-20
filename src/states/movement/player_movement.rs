use crate::states::player::*;

impl Player {
    pub fn move_player(&mut self, gravity: f32, delta_time: f32) {
        self.movement.update(&self.inputs.direction, gravity, delta_time);
    }
}