use macroquad::prelude::*;
use crate::states::Player;

impl Player {
    pub fn draw_player(&self, central_coordinates: (f32, f32)) {
        let (player_x, player_y) = self.movement.position.into(); // Convert Vec2 to tuple of f32 (x, y)
        let (camera_x, camera_y) = central_coordinates;

        let width = 128.0;
        let height = 256.0;

        // Calculate screen edges based on center focus point
        let screen_left = camera_x - (crate::LOGICAL_WIDTH / 2.0);
        let screen_top = camera_y - (crate::LOGICAL_HEIGHT / 2.0);

        // Track relative to screen edges, centering the player's waist
        let screen_x = (player_x - screen_left - (width / 2.0)).round();
        let screen_y = (player_y - screen_top - (height / 2.0)).round();

        if let Some(texture) = crate::sprites::get("entity:player") {
            draw_texture_ex(texture, screen_x, screen_y, WHITE, DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                ..Default::default()
            });
        }
    }
}