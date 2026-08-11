// Import file types
use super::super::tile::Tile;
use super::super::tile::TileType;
use crate::states::environment::terrain::hills::*;

pub struct Scene {
    pub scene_id: String,
    pub name: String,
    pub description: String,
    pub starts: Vec<(u32, u32)>,
    pub tiles: SceneMap,
}
impl Scene {
    pub fn new(scene_id: String, name: String, description: String, size: (u32, u32)) -> Self {
        Self {
            scene_id,
            name,
            description,
            starts: Vec::new(),
            tiles: generate_hills(size.0 as usize, size.1 as usize, 3),
        }
    }
    pub fn add_start(&mut self, position: (u32, u32)) {
        self.starts.push(position);
    }
}
#[derive(Debug)]
pub struct SceneMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<Tile>>,
}
impl SceneMap {
    pub fn new(width: u32, height: u32) -> Self {
        // Pre-allocate the memory for our rows to keep initialization fast
        let mut tiles = Vec::with_capacity(height as usize);
        
        for y in 0..(height as usize) {
            // Pre-allocate memory for the columns in this row
            let mut row = Vec::with_capacity(width as usize);
            for x in 0..(width as usize) {
                // Initialize each tile dynamically with its exact X and Y coordinates
                row.push(Tile::new("utility:air".to_string(), x, y, crate::TILE_SIZE));
            }
            tiles.push(row);
        }

        Self {
            width,
            height,
            tiles,
        }
    }
}

use macroquad::prelude::*;

impl SceneMap {
    /// Resolves kinematic AABB collision detection and response against static solid tiles in the scene.
    ///
    /// Collision resolution is performed per-axis (`checking_x`) using an insulated collision frame 
    /// ("skin") to prevent orthogonal cross-talk (e.g., snagging on floor seams while moving sideways).
    ///
    /// # Arguments
    ///
    /// * `bounds` - A mutable reference to the entity's bounding box (`Rect`). Position coordinates (`x` or `y`) will be updated upon collision.
    /// * `velocity` - A mutable reference to the entity's velocity (`Vec2`). The corresponding axis component is zeroed upon impact.
    /// * `checking_x` - `true` to check horizontal movement; `false` to check vertical movement.
    /// * `is_grounded` - An optional mutable reference to a grounded flag. Set to `true` when colliding downward against a solid top face.
    ///
    /// # Example
    ///
    /// ```rust
    /// // Horizontal pass
    /// scene_map.handle_collisions(&mut entity_rect, &mut entity_vel, true, None);
    /// 
    /// // Vertical pass
    /// scene_map.handle_collisions(&mut entity_rect, &mut entity_vel, false, Some(&mut entity.is_grounded));
    /// ```
    pub fn handle_collisions(
        &self,
        bounds: &mut Rect,
        velocity: &mut Vec2,
        checking_x: bool,
        mut is_grounded: Option<&mut bool>,
    ) {
        // A tiny floating-point safety buffer in block units (no raw pixels!)
        let skin = 0.02;

        // 1. Inset the OPPOSITE axis of motion to prevent ground/wall cross-talk
        let search_rect = if checking_x {
            Rect::new(
                bounds.x,
                bounds.y + skin,
                bounds.w,
                bounds.h - (skin * 2.0),
            )
        } else {
            Rect::new(
                bounds.x + skin,
                bounds.y,
                bounds.w - (skin * 2.0),
                bounds.h,
            )
        };

        // 2. Calculate dynamic grid boundaries based on the insulated rect footprint
        let start_x = search_rect.left().floor() as i32 - 1;
        let end_x = search_rect.right().ceil() as i32 + 1;
        let start_y = search_rect.top().floor() as i32 - 1;
        let end_y = search_rect.bottom().ceil() as i32 + 1;

        let check_start_x = start_x.max(0) as usize;
        let check_end_x = (end_x.max(0) as usize).min(self.width as usize);
        let check_start_y = start_y.max(0) as usize;
        let check_end_y = (end_y.max(0) as usize).min(self.height as usize);

        // 3. Scan the insulated block grid path
        for y in check_start_y..check_end_y {
            for x in check_start_x..check_end_x {
                let tile = &self.tiles[y][x];
                if !tile.tile_data.is_solid {
                    continue;
                }

                // Uniform 1.0 x 1.0 static tile bounds
                let tile_rect = Rect::new(x as f32, y as f32, 1.0, 1.0);

                // 4. Regenerate the checking frame using current target coordinates
                let current_box = if checking_x {
                    Rect::new(
                        bounds.x,
                        bounds.y + skin,
                        bounds.w,
                        bounds.h - (skin * 2.0),
                    )
                } else {
                    Rect::new(
                        bounds.x + skin,
                        bounds.y,
                        bounds.w - (skin * 2.0),
                        bounds.h,
                    )
                };

                if current_box.overlaps(&tile_rect) {
                    if checking_x {
                        if velocity.x > 0.0 {
                            // Snaps right edge of target body against the left block face
                            bounds.x = tile_rect.left() - bounds.w;
                        } else if velocity.x < 0.0 {
                            // Snaps left edge of target body against the right block face
                            bounds.x = tile_rect.right();
                        }
                        velocity.x = 0.0;
                    } else {
                        if velocity.y > 0.0 {
                            // Snaps bottom edge of target body against the top block face
                            bounds.y = tile_rect.top() - bounds.h;
                            if let Some(ref mut grounded) = is_grounded {
                                **grounded = true;
                            }
                        } else if velocity.y < 0.0 {
                            // Snaps top edge of target body against the bottom block face
                            bounds.y = tile_rect.bottom();
                        }
                        velocity.y = 0.0;
                    }
                }
            }
        }
    }
}