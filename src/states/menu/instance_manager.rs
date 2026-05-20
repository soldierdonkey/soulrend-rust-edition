use macroquad::prelude::*;
use crate::runtime::Runtime;
use crate::states::{GameState, InGameState};
pub fn instance_manager(game: &mut Runtime) {
    draw_text("Press Space to enter the instance!", 0.0, 40.0, 80.0, RED);
    draw_text("Press Escape to exit to the main menu!", 0.0, 150.0, 80.0, RED);
    if is_key_pressed(KeyCode::Space) {
        // Initialize the inner state data when transitioning
        game.current_state = GameState::InstanceCreator;
    }
    if is_key_pressed(KeyCode::Escape) {
        // Initialize the inner state data when transitioning
        game.current_state = GameState::MainMenu;
    }
}