use macroquad::math::Vec2;
use macroquad::prelude::*;
use crate::states::GameState;

pub struct Runtime {
    pub current_state: GameState,
}
impl Runtime {
    pub fn new() -> Self {
        Runtime {
            current_state: GameState::MainMenu,
        }
    }
}

pub struct Inputs {
    pub mouse_position: Vec2
}
impl Inputs {
    pub fn new() -> Self{
        Inputs {
            mouse_position: Vec2::ZERO
        }
    }
    pub fn update(&mut self) {
        // 1. Get current window dimensions
        let window_w = screen_width();
        let window_h = screen_height();

        // 2. Re-calculate the exact same letterbox scaling/offsets used in your render loop
        let scale = (window_w / crate::VIRTUAL_WIDTH).min(window_h / crate::VIRTUAL_HEIGHT);
        let x_offset = (window_w - (crate::VIRTUAL_WIDTH * scale)) / 2.0;
        let y_offset = (window_h - (crate::VIRTUAL_HEIGHT * scale)) / 2.0;

        // 3. Get raw mouse position
        let (raw_x, raw_y) = mouse_position();

        // 4. Subtract the black bar offsets, then divide by the scale
        let logical_x = (raw_x - x_offset) / scale;
        let logical_y = (raw_y - y_offset) / scale;

        self.mouse_position = vec2(logical_x, logical_y);
    }
}

pub fn get_mouse_position() -> Vec2 {
    crate::INPUT.lock().unwrap().mouse_position.clone()
}