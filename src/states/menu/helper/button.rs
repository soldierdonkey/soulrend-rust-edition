use crate::runtime::Runtime;
use crate::sprites::get;
use macroquad::prelude::*;
use macroquad::math::Vec2;

impl Runtime {
    pub fn button(&mut self, sprite_id: &str, x: f32, y: f32) -> bool {
        match get(sprite_id) {
            Some(sprite) => {
                // println!("Rect for button '{}': x={}, y={}, width={}, height={}", sprite_id, x, y, sprite.width(), sprite.height());
                // println!("Mouse position: x={}, y={}", self.mouse_position.x, self.mouse_position.y);
                let rect = Rect::new(x, y, sprite.width() as f32, sprite.height() as f32);
                if rect.contains(Vec2::new(self.mouse_position.x, self.mouse_position.y)) {
                    // println!("Hovering over button '{}'", sprite_id);
                    draw_texture(sprite, x, y, WHITE);
                    if is_mouse_button_pressed(MouseButton::Left) {
                        return true;
                    }
                } else {
                    draw_texture(sprite, x, y, GRAY);
                }
            }
            None => {
                panic!("Sprite with ID '{}' not found in registry!", sprite_id);
            }
        }
        false
    }
}