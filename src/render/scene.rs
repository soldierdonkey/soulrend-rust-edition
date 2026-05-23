use macroquad::prelude::*;
use crate::render;
use crate::assets::*;
use crate::states::environment::tile::TileType;
use crate::states::environment::SceneMap;
use crate::states::environment::Scene;

impl Scene {
    /// Entry point to draw the entire scene
    pub fn draw(&self, player_coordinates: (f32, f32)) {
        // You can add logic here later to draw background layers or parallax
        // println!("Drawing scene '{}' with player at coordinates: {:?}", self.name, player_coordinates);
        self.tiles.draw(player_coordinates);
    }
}

impl SceneMap {
    pub fn draw(&self, central_coordinates: (f32, f32)) {
        let (camera_x, camera_y) = central_coordinates; // Both are in block units now
        
        // Convert screen width and height constants from pixel space into block units
        let view_width_blocks = crate::LOGICAL_WIDTH / crate::TILE_SIZE;
        let view_height_blocks = crate::LOGICAL_HEIGHT / crate::TILE_SIZE;
        
        // 1. Derive the initial top-left corner of the screen in block units
        let mut screen_left = camera_x - (view_width_blocks / 2.0);
        let mut screen_top = camera_y - (view_height_blocks / 2.0);

        // 2. Calculate the maximum permissible top-left boundary limits in blocks
        let max_screen_left = self.width as f32 - view_width_blocks;
        let max_screen_top = self.height as f32 - view_height_blocks;

        // 3. Clamp values to prevent rendering out-of-bounds negative space
        screen_left = screen_left.clamp(0.0, max_screen_left.max(0.0));
        screen_top = screen_top.clamp(0.0, max_screen_top.max(0.0));

        // Re-derive screen right and bottom bounds in blocks
        let screen_right = screen_left + view_width_blocks;
        let screen_bottom = screen_top + view_height_blocks;

        // 4. Bounding indices can be converted directly without division mutations
        let mut start_x = screen_left.floor() as i32 - 1;
        let mut end_x = screen_right.ceil() as i32 + 1;
        let mut start_y = screen_top.floor() as i32 - 1;
        let mut end_y = screen_bottom.ceil() as i32 + 1;

        // 5. Hard clamp the matrix indices to map limits to eliminate out-of-bounds panics
        start_x = start_x.clamp(0, self.width as i32);
        end_x = end_x.clamp(0, self.width as i32);
        start_y = start_y.clamp(0, self.height as i32);
        end_y = end_y.clamp(0, self.height as i32);

        // Render loop
        for y in (start_y as usize)..(end_y as usize) {
            for x in (start_x as usize)..(end_x as usize) {
                let tile = &self.tiles[y][x];
                if let TileType::Empty = tile.tile_type { continue; }

                // Translate grid indexing to logic offset bounds, then multiply by pixel scale
                let world_x = ((x as f32 - screen_left) * crate::TILE_SIZE).round();
                let world_y = ((y as f32 - screen_top) * crate::TILE_SIZE).round();

                let sprite_key = match &tile.tile_type {
                    TileType::Solid(texture_path) => texture_path.as_str(),
                    _ => "soulrend:unknown",
                };

                if let Some(texture) = crate::assets::sprites::get(sprite_key) {
                    draw_texture_ex(
                        texture,
                        world_x,
                        world_y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(crate::TILE_SIZE, crate::TILE_SIZE)),
                            ..Default::default()
                        },
                    );
                } else {
                    draw_rectangle(world_x, world_y, crate::TILE_SIZE, crate::TILE_SIZE, MAGENTA);
                }
            }
        }
    }
}