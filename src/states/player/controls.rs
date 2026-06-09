use macroquad::prelude::*;
use crate::states::player::*;

pub struct Actions {
    pub direction: Direction,
    pub vertical: Direction,
    pub dash: bool
}

impl Actions {
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
            self.actions.vertical = Direction::Up;
        } else if is_key_down(KeyCode::S) {
            self.actions.vertical = Direction::Down;
        } else {
            self.actions.vertical = Direction::None;
        }
        if is_key_down(KeyCode::A) && is_key_down(KeyCode::D) {
            self.actions.direction = Direction::None;
        } else if is_key_down(KeyCode::A) {
            self.actions.direction = Direction::Left;
        } else if is_key_down(KeyCode::D) {
            self.actions.direction = Direction::Right;
        } else {
            self.actions.direction = Direction::None;
        }
        self.actions.dash = is_key_pressed(KeyCode::R)
    }
}