use macroquad::prelude::*;
use crate::runtime::Runtime;
use crate::states::{GameState, InGameState};
pub fn main_menu(game: &mut Runtime) {
    game.render_screen("soulrend:background/main_menu".to_string(), vec2(0.0, 0.0));
}