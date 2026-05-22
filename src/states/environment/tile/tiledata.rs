use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct TileData {
    pub bounds: Rect,
    pub is_solid: bool, // Useful to distinguish between solid ground and background walls
    // Add other fields here later (e.g., texture ID, variant)
}
impl TileData {
    pub fn new(x: f32, y: f32, size: f32, is_solid: bool) -> Self {
        Self {
            bounds: Rect::new(x, y, size, size),
            is_solid,
        }
    }
}