use macroquad::prelude::*;
use crate::states::player::*;
use crate::states::movement::*;

pub struct Inputs {
    pub direction: Direction,
    pub vertical: Direction,
    pub dash: bool
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            direction: Direction::None,
            vertical: Direction::None,
            dash: false
        }
    }
}

impl Player {
    pub fn update_controls(&mut self) {
        if is_key_down(KeyCode::W) {
            self.inputs.vertical = Direction::Up;
        } else if is_key_down(KeyCode::S) {
            self.inputs.vertical = Direction::Down;
        } else {
            self.inputs.vertical = Direction::None;
        }
        if is_key_down(KeyCode::A) && is_key_down(KeyCode::D) {
            self.inputs.direction = Direction::None;
        } else if is_key_down(KeyCode::A) {
            self.inputs.direction = Direction::Left;
        } else if is_key_down(KeyCode::D) {
            self.inputs.direction = Direction::Right;
        } else {
            self.inputs.direction = Direction::None;
        }
        self.inputs.dash = is_key_pressed(KeyCode::R)
    }
}