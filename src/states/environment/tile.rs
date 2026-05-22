// src/states/environment/tile.rs
// import tile data (stuff that is uniqe for tiles)
mod tiledata;
pub use self::tiledata::*;
// import tile type (shared traits of tiles)
mod tiletype;
pub use self::tiletype::*;

#[derive(Clone, Debug)]
pub struct Tile {
    pub tile_data: TileData,
    pub tile_type: TileType
}
impl Tile {
    pub fn new(tile_type: TileType, grid_x: usize, grid_y: usize, tile_size: f32) -> Self {
        // Automatically determine if the tile is solid based on its type
        let is_solid = match &tile_type {
            TileType::Empty => false,
            TileType::Solid(_) => true, 
            _ => false, // Add other match arms as your TileType enum grows
        };

        // Convert the grid indices (0, 1, 2) into actual world pixel coordinates (0.0, 32.0, 64.0)
        let world_x = grid_x as f32 * tile_size;
        let world_y = grid_y as f32 * tile_size;

        Self {
            tile_data: TileData::new(world_x, world_y, tile_size, is_solid),
            tile_type,
        }
    }
    pub fn update(&mut self, tile_type: TileType) {
        self.tile_type = tile_type;
        self.tile_data.is_solid = match &self.tile_type {
            TileType::Empty => false,
            TileType::Solid(_) => true,
            _ => false,
        };
    }
}