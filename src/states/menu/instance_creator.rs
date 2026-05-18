use macroquad::prelude::*;
use crate::runtime::Runtime;
use crate::states::{GameState, InGameState};
pub fn instance_creator(game: &mut Runtime) {
    draw_text("Press Space to create an instance!", 0.0, 10.0, 20.0, RED);
    draw_text("Press Escape to exit to the manager!", 0.0, 30.0, 20.0, RED);
    if is_key_pressed(KeyCode::Space) {
        // Initialize the inner state data when transitioning
        game.current_state = GameState::InGame(InGameState::new());
    }
    if is_key_pressed(KeyCode::Escape) {
        // Initialize the inner state data when transitioning
        game.current_state = GameState::InstanceManager;
    }
}