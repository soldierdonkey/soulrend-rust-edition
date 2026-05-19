use macroquad::prelude::*;
use crate::runtime::Runtime;
use crate::states::{GameState, InGameState};
use super::movement::Direction;
impl InGameState {
    pub fn in_game(&mut self) {
        self.player.movement.update(Direction::Left, 1.0, get_frame_time());
        // println!("Player position: x={}, y={}", self.player.movement.position.x, self.player.movement.position.y);
        self.environment.worlds[self.player.world].scenes[self.player.scene].draw((self.player.movement.position.x, self.player.movement.position.y));
    }
} 