use macroquad::prelude::*;
use crate::states::Player;

impl Player {
    pub fn draw_player(&self, central_coordinates: (f32, f32)) {
        // Use the actual physics top-left position
        let (player_x, player_y) = (self.movement.position.x, self.movement.position.y);
        let (camera_x, camera_y) = central_coordinates;

        // Keep your constants for rendering size
        let visual_width = 128.0;
        let visual_height = 256.0;

        let screen_left = camera_x - (crate::LOGICAL_WIDTH / 2.0);
        let screen_top = camera_y - (crate::LOGICAL_HEIGHT / 2.0);

        // Apply an offset to draw the player's waist centered
        // We are drawing the texture slightly left/up so the physics box (top-left) 
        // aligns with the waist.
        let waist_offset_x = (visual_width - self.movement.size.x) / 2.0;
        let waist_offset_y = (visual_height - self.movement.size.y) / 2.0;

        let screen_x = (player_x - screen_left - waist_offset_x).round();
        let screen_y = (player_y - screen_top - waist_offset_y).round();

        if let Some(texture) = crate::sprites::get("entity:player") {
            draw_texture_ex(texture, screen_x, screen_y, WHITE, DrawTextureParams {
                dest_size: Some(vec2(visual_width, visual_height)),
                ..Default::default()
            });
        }
    }
}