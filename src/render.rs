use macroquad::prelude::*;
use crate::sprites::*;
use crate::states::environment::tile::TileType;
use crate::states::environment::SceneMap;
use crate::states::environment::Scene;
pub const TILE_SIZE: f32 = 32.0;

impl Scene {
    /// Entry point to draw the entire scene
    pub fn draw(&self, player_coordinates: (f32, f32)) {
        // You can add logic here later to draw background layers or parallax
        // println!("Drawing scene '{}' with player at coordinates: {:?}", self.name, player_coordinates);
        self.tiles.draw(player_coordinates);
    }
}

impl SceneMap {
    pub fn draw(&self, player_coordinates: (f32, f32)) {
        // Iterate through rows (Y) and then columns (X)
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let tile = &self.tiles[y][x];

                // 1. Map grid indices to world pixel coordinates
                let world_x = x as f32 * TILE_SIZE - player_coordinates.0 ;
                let world_y = y as f32 * TILE_SIZE - player_coordinates.1 ;

                // 2. Map your TileType to a specific sprite namespace:id
                // (Adjust 'tile.tile_type' to whatever field/method your Tile uses)
                let sprite_key = match &tile.tile_type {
                    TileType::Empty => continue, // Skip rendering empty air to save frames!
                    TileType::Solid(key) => key,
                    // Add your other tile types here...
                    _ => continue, 
                };
                println!("Drawing tile at grid ({}, {}) with sprite '{}'", x, y, sprite_key);
                // 3. Draw the tile
                if let Some(texture) = get(&sprite_key) {
                    draw_texture(texture, world_x, world_y, WHITE);
                } else {
                    // Fallback: If the sprite forgot to load or doesn't exist, 
                    // draw a highly visible magenta box so you know it's broken.
                    draw_rectangle(world_x, world_y, TILE_SIZE, TILE_SIZE, MAGENTA);
                }
            }
        }
    }
}