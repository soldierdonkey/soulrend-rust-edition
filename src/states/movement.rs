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
use crate::helper::approach::approach;

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
    pub fn new(start_x: f32, start_y: f32) -> Self {
        Self {
            position: Vec2::new(start_x, start_y),
            velocity: Vec2::new(0.0, 0.0),
            acceleration: 50.0,
            is_grounded: false,
            friction: 25.0, 
            size: Vec2::new(1.0, 2.0), // Standard clean proportions (60% width, 1.8 blocks high)
        }
    }

    pub fn update(&mut self, actions: &Actions, scene_map: &SceneMap, dt: f32) {
        let gravity = 22.0;       // Blocks per second downward acceleration
        let walk_speed = 7.0;     // Run velocity limit in blocks
        let jump_force = -11.0;   // Immediate negative vertical impulse

        // --- Horizontal Input Handling ---
        match actions.direction {
            Direction::Left => self.velocity.x = -walk_speed,
            Direction::Right => self.velocity.x = walk_speed,
            Direction::None => {
                self.velocity.x = approach(self.velocity.x, 0.0, self.friction * dt);
            }
            _ => {}
        }
        
        // --- Jump Input Handling ---
        if actions.vertical == Direction::Up && self.is_grounded {
            self.velocity.y = jump_force;
            self.is_grounded = false;
        }

        // --- Apply Ambient Gravity ---
        self.velocity.y += gravity * dt;

        // --- Move & Resolve Collision: X-Axis ---
        self.position.x += self.velocity.x * dt;
        self.handle_collisions(scene_map, true);

        // --- Move & Resolve Collision: Y-Axis ---
        self.position.y += self.velocity.y * dt;
        self.is_grounded = false; 
        self.handle_collisions(scene_map, false);
    }
    fn handle_collisions(&mut self, scene_map: &SceneMap, checking_x: bool) {
        // A tiny floating-point safety buffer in block units (no raw pixels!)
        let skin = 0.02; 

        // 1. Inset the OPPOSITE axis of motion to prevent ground/wall cross-talk
        let mut search_rect = if checking_x {
            // Checking walls: shrink vertically so feet/head don't catch floors/ceilings
            Rect::new(
                self.position.x,
                self.position.y + skin,
                self.size.x,
                self.size.y - (skin * 2.0),
            )
        } else {
            // Checking floors: shrink horizontally so sides don't catch wall surfaces
            Rect::new(
                self.position.x + skin,
                self.position.y,
                self.size.x - (skin * 2.0),
                self.size.y,
            )
        };

        // 2. Calculate dynamic grid boundaries based on the insulated rect footprint
        let start_x = search_rect.left().floor() as i32 - 1;
        let end_x = search_rect.right().ceil() as i32 + 1;
        let start_y = search_rect.top().floor() as i32 - 1;
        let end_y = search_rect.bottom().ceil() as i32 + 1;

        let check_start_x = start_x.max(0) as usize;
        let check_end_x = (end_x.max(0) as usize).min(scene_map.width as usize);
        let check_start_y = start_y.max(0) as usize;
        let check_end_y = (end_y.max(0) as usize).min(scene_map.height as usize);

        // 3. Scan the insulated block grid path
        for y in check_start_y..check_end_y {
            for x in check_start_x..check_end_x {
                let tile = &scene_map.tiles[y][x];
                if !tile.tile_data.is_solid { continue; }

                // Uniform 1.0 x 1.0 static tile bounds
                let tile_rect = Rect::new(x as f32, y as f32, 1.0, 1.0);

                // 4. Regenerate the checking frame using the player's updated position coordinates
                let current_player_box = if checking_x {
                    Rect::new(self.position.x, self.position.y + skin, self.size.x, self.size.y - (skin * 2.0))
                } else {
                    Rect::new(self.position.x + skin, self.position.y, self.size.x - (skin * 2.0), self.size.y)
                };

                if current_player_box.overlaps(&tile_rect) {
                    if checking_x {
                        if self.velocity.x > 0.0 {
                            // Snaps left side of player body right against the block face
                            self.position.x = tile_rect.left() - self.size.x;
                        } else if self.velocity.x < 0.0 {
                            self.position.x = tile_rect.right();
                        }
                        self.velocity.x = 0.0;
                    } else {
                        if self.velocity.y > 0.0 {
                            self.position.y = tile_rect.top() - self.size.y;
                            self.is_grounded = true;
                        } else if self.velocity.y < 0.0 {
                            self.position.y = tile_rect.bottom();
                        }
                        self.velocity.y = 0.0;
                    }
                }
            }
        }
    }
}