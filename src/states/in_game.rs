use macroquad::prelude::*;
use crate::debug;
use crate::runtime::Runtime;
use crate::states::{GameState, InGameState};
use super::movement::Direction;
use crate::render::player::*;

impl InGameState {
    pub fn in_game(&mut self) {
        if debug::debug_frame() {
            // println!("{:?}", &self.environment.worlds[self.player.world].scenes[self.player.scene].tiles);
            println!("Player velocity: x={}, y={}, Player position: x={}, y={}", self.player.creature.movement.velocity.x, self.player.creature.movement.velocity.y, self.player.creature.movement.position.x, self.player.creature.movement.position.y);
        }
        self.player.update_controls();
        self.player.move_player(&self.environment.worlds[self.player.creature.world].scenes[self.player.creature.scene].tiles, get_frame_time());
        // println!("Player position: x={}, y={}", self.player.movement.position.x, self.player.movement.position.y);
        let camera_center_x = self.player.creature.movement.position.x + (self.player.creature.movement.size.x / 2.0);
        let camera_center_y = self.player.creature.movement.position.y + (self.player.creature.movement.size.y / 2.0);

        self.environment.worlds[self.player.creature.world].scenes[self.player.creature.scene].draw((
            camera_center_x,
            camera_center_y
        ));
        self.player.draw_player(
            (camera_center_x, camera_center_y), 
            self.environment.worlds[self.player.creature.world].scenes[self.player.creature.scene].tiles.width as usize, 
            self.environment.worlds[self.player.creature.world].scenes[self.player.creature.scene].tiles.height as usize
        );
        // self.player.creature.kinematics.update(get_frame_time());
    }
} 