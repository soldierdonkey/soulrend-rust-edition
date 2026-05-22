use macroquad::prelude::*;
use crate::render;
use crate::sprites::*;
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
        let (camera_x, camera_y) = central_coordinates; 
        
        // 1. Derive the initial top-left corner of the screen by subtracting half the screen size from the center focus
        let mut screen_left = camera_x - (crate::LOGICAL_WIDTH / 2.0);
        let mut screen_top = camera_y - (crate::LOGICAL_HEIGHT / 2.0);

        // 2. Calculate the maximum permissible top-left limits based on total world pixels
        let max_screen_left = (self.width as f32 * crate::TILE_SIZE) - crate::LOGICAL_WIDTH;
        let max_screen_top = (self.height as f32 * crate::TILE_SIZE) - crate::LOGICAL_HEIGHT;

        // 3. Clamp camera matrix offsets to prevent viewing negative space or scrolling past world boundaries
        // max(0.0) shields the layout calculation if a small test map size is narrower than the viewport capacity
        screen_left = screen_left.clamp(0.0, max_screen_left.max(0.0));
        screen_top = screen_top.clamp(0.0, max_screen_top.max(0.0));

        // Re-derive screen right and bottom from our perfectly clamped top-left anchor
        let screen_right = screen_left + crate::LOGICAL_WIDTH;
        let screen_bottom = screen_top + crate::LOGICAL_HEIGHT;

        // 4. Convert pixel space boundaries safely to tile matrix grid indices
        let mut start_x = (screen_left / crate::TILE_SIZE).floor() as i32 - 1;
        let mut end_x = (screen_right / crate::TILE_SIZE).ceil() as i32 + 1;
        let mut start_y = (screen_top / crate::TILE_SIZE).floor() as i32 - 1;
        let mut end_y = (screen_bottom / crate::TILE_SIZE).ceil() as i32 + 1;

        // 5. Hard clamp the matrix indices to world limits to eliminate out-of-bounds panics
        start_x = start_x.clamp(0, self.width as i32);
        end_x = end_x.clamp(0, self.width as i32);
        start_y = start_y.clamp(0, self.height as i32);
        end_y = end_y.clamp(0, self.height as i32);

        // 3. Render loop
        for y in (start_y as usize)..(end_y as usize) {
            for x in (start_x as usize)..(end_x as usize) {
                let tile = &self.tiles[y][x];
                if let TileType::Empty = tile.tile_type { continue; }

                // Camera matrix projection using raw floats, THEN rounded to 
                // lock perfectly into the 1920x1080 logical pixel grid.
                let world_x = (x as f32 * crate::TILE_SIZE - screen_left).round();
                let world_y = (y as f32 * crate::TILE_SIZE - screen_top).round();

                let sprite_key = match &tile.tile_type {
                    TileType::Solid(texture_path) => texture_path.as_str(),
                    _ => "soulrend:unknown",
                };

                if let Some(texture) = crate::sprites::get(sprite_key) {
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
                // draw_text((format!("{}, {}", x, y)).as_str(), world_x, world_y, 40.0, RED);
                // println!("Drawing tile at coordinates: {}, {}", x, y);
            }
        }
    }
}