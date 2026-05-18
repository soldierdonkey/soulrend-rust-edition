// src/states/movement.rs
// Direction Enum
// Implements its own None so Option<Direction> is not needed. Does not carry data.
use super::super::helper::approach;
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
// Position Struct
pub struct Movement {
    position: Position,
    velocity: Velocity,
    acceleration: f32,
    is_grounded: bool,
    friction: f32,
}
impl Movement {
    pub fn new() -> Self {
        Self {
            position: Position { x: 0.0, y: 0.0 },
            velocity: Velocity { x: 0.0, y: 0.0 },
            acceleration: 0.0,
            is_grounded: true,
            friction: 10.0, // Friction coefficient for deceleration when no input is
        }
    }
    pub fn update(&mut self, direction: Direction, gravity: f32, delta_time: f32) {
        // Update velocity based on direction and acceleration
        match direction {
            Direction::Left => self.velocity.x -= self.acceleration * delta_time,
            Direction::Right => self.velocity.x += self.acceleration * delta_time,
            Direction::None => self.velocity.x = approach(self.velocity.x, 0.0, self.friction * delta_time),
            _ => {panic!("Only Left, Right, and None directions should affect horizontal movement!")}
        }
        if !self.is_grounded {
            self.velocity.y += gravity * delta_time;
        }
        // Update position based on velocity and delta_time
    }
}
pub struct Position {
    x: f32,
    y: f32,
}
pub struct Velocity {
    x: f32,
    y: f32,
}