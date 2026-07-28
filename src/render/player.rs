use macroquad::prelude::*;
use crate::states::Player;
use crate::assets;

impl Player {
    // If you keep the signature identical, pass map_width and map_height so the player 
    // code can perform the identical camera-boundary edge clamping calculations.
    pub fn draw_player(&self, central_coordinates: (f32, f32), map_width: usize, map_height: usize) {
        let registry = match assets::kinematics::get("soulrend:test") {
            Some(data) => data,
            None => return,
        };
        let (player_x, player_y) = (self.creature.movement.position.x, self.creature.movement.position.y); // in block units
        let (camera_x, camera_y) = central_coordinates;

        // Render target pixel sizes for high-res combat sprite sheets
        let visual_width = registry.aabb_width;
        let visual_height = registry.aabb_height;

        let view_width_blocks = crate::LOGICAL_WIDTH / crate::TILE_SIZE;
        let view_height_blocks = crate::LOGICAL_HEIGHT / crate::TILE_SIZE;

        // Emulate the exact camera viewport offset mapping used by the tile map
        let mut screen_left = camera_x - (view_width_blocks / 2.0);
        let mut screen_top = camera_y - (view_height_blocks / 2.0);

        let max_screen_left = map_width as f32 - view_width_blocks;
        let max_screen_top = map_height as f32 - view_height_blocks;

        screen_left = screen_left.clamp(0.0, max_screen_left.max(0.0));
        screen_top = screen_top.clamp(0.0, max_screen_top.max(0.0));

        // Scale your physics bounding dimensions into matching pixel structures
        let physics_w_px = registry.aabb_width;
        let physics_h_px = registry.aabb_height;

        // Apply visual buffer offsets so larger combat sprites frame the rigid bounding box accurately
        let waist_offset_x = (visual_width - physics_w_px) / 2.0;
        let waist_offset_y = (visual_height - physics_h_px) / 2.0;

        // Project logic positions to screenspace coordinates, subtract visual offsets, and round
        let screen_x = (((player_x - screen_left) * crate::TILE_SIZE) - waist_offset_x).round();
        let screen_y = (((player_y - screen_top) * crate::TILE_SIZE) - waist_offset_y).round();

        // if let Some(texture) = crate::assets::sprites::get("entity:player") {
        //     draw_texture_ex(texture, screen_x, screen_y, WHITE, DrawTextureParams {
        //         dest_size: Some(vec2(visual_width, visual_height)),
        //         ..Default::default()
        //     });
        // }
        self.creature.kinematics.draw(Vec2::new(screen_x, screen_y));
        // Player coordinates:
        // draw_circle(screen_x, screen_y, 10.0, YELLOW);
    }
}