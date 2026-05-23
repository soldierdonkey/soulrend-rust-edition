use macroquad::prelude::*;
use crate::runtime::Runtime;
use crate::states::{GameState, InGameState};
pub fn main_menu(game: &mut Runtime) {
    if let Some(texture) = crate::assets::sprites::get("menu:main") {
        draw_texture(texture, 0.0, 0.0, WHITE);
    }
    if game.button("menu:play", 480.0, 720.0) {
        game.current_state = GameState::InstanceManager;
    }
}