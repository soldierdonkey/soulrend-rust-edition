use macroquad::math::Vec2;
use macroquad::prelude::*;
use crate::states::player::controls::*;
use crate::states::environment::tile::*;
use crate::states::environment::scene::*;

mod player_movement;
pub use self::player_movement::*;
// src/states/movement.rs
// Direction Enum
// Implements its own None so Option<Direction> is not needed. Does not carry data.
use super::super::helper::approach;

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
// Position Struct
pub struct Movement {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: f32,
    pub is_grounded: bool,
    pub friction: f32,
    pub size: Vec2,
}
impl Movement {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            acceleration: 500.0,
            is_grounded: false,
            friction: 10.0, // Friction coefficient for deceleration when no input is
            size: Vec2::new(128.0, 256.0),
        }
    }
    // Use this for collision detection only!
    // It creates a slightly smaller box centered inside the player.
    pub fn get_collision_rect(&self) -> Rect {
        let padding = 4.0; // Shrink by 4 pixels on all sides
        Rect::new(
            self.position.x + padding, 
            self.position.y + padding, 
            self.size.x - (padding * 2.0), 
            self.size.y - padding // Keep the bottom slightly longer to ensure ground detection
        )
    }
    pub fn update(&mut self, inputs: &Inputs, scene_map: &SceneMap, dt: f32) {
        let gravity = 1200.0;
        let walk_speed = 300.0;
        let jump_force = -550.0;

        // --- Inputs ---
        // Do NOT reset velocity to 0.0 unconditionally here.
        match inputs.direction {
            Direction::Left => self.velocity.x = -walk_speed,
            Direction::Right => self.velocity.x = walk_speed,
            Direction::None => {
                self.velocity.x = approach(self.velocity.x, 0.0, self.friction * dt);
            }
            _ => {}
        }
        
        // --- Jumping ---
        if inputs.vertical == Direction::Up && self.is_grounded {
            self.velocity.y = jump_force;
            self.is_grounded = false;
        }

        // --- Gravity ---
        self.velocity.y += gravity * dt;

        // --- Move & Collide X ---
        self.position.x += self.velocity.x * dt;
        self.handle_collisions(scene_map, true);

        // --- Move & Collide Y ---
        self.position.y += self.velocity.y * dt;
        self.is_grounded = false; // Will be set to true inside handle_collisions if we hit a floor
        self.handle_collisions(scene_map, false);
    }

    fn handle_collisions(&mut self, scene_map: &SceneMap, checking_x: bool) {
        // 1. Use the new inset collision box
        let player_rect = self.get_collision_rect();

        // 1. Calculate boundaries in grid indices
        // Clamp values to ensure we stay within the tile vector range
        let start_x = (player_rect.left() / crate::TILE_SIZE).floor().clamp(0.0, scene_map.width as f32 - 1.0) as usize;
        let end_x = (player_rect.right() / crate::TILE_SIZE).floor().clamp(0.0, scene_map.width as f32 - 1.0) as usize;
        
        let start_y = (player_rect.top() / crate::TILE_SIZE).floor().clamp(0.0, scene_map.height as f32 - 1.0) as usize;
        let end_y = (player_rect.bottom() / crate::TILE_SIZE).floor().clamp(0.0, scene_map.height as f32 - 1.0) as usize;

        // 2. Iterate only over the relevant grid slice
        for y in start_y..=end_y {
            for x in start_x..=end_x {
                let tile = &scene_map.tiles[y][x];
                if !tile.tile_data.is_solid { continue; }

                if player_rect.overlaps(&tile.tile_data.bounds) {
                    // Only resolve if we are actually moving into the wall
                    if checking_x {
                        println!("Collision detected in X direction");
                        if self.velocity.x > 0.0 {
                            // Subtract a tiny "skin" (e.g., 0.1) so we aren't perfectly flush
                            self.position.x = &tile.tile_data.bounds.left() - self.size.x - 0.1;
                        } else if self.velocity.x < 0.0 {
                            self.position.x = &tile.tile_data.bounds.right() + 0.1;
                        }
                        self.velocity.x = 0.0;
                    } else {
                        if self.velocity.y > 0.0 {
                            self.position.y = &tile.tile_data.bounds.top() - self.size.y - 0.1;
                            self.is_grounded = true;
                        } else if self.velocity.y < 0.0 {
                            self.position.y = &tile.tile_data.bounds.bottom() + 0.1;
                        }
                        self.velocity.y = 0.0;
                    }
                }
            }
        }
    }
}