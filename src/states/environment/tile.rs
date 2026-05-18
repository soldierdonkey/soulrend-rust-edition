// src/states/environment/tile.rs
// import tile data (stuff that is uniqe for tiles)
mod tiledata;
pub use self::tiledata::*;
// import tile type (shared traits of tiles)
mod tiletype;
pub use self::tiletype::*;

#[derive(Clone)]
pub struct Tile {
    pub tile_data: TileData,
    pub tile_type: TileType
}
impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self {
            tile_data: TileData::new(),
            tile_type: TileType::Empty,
        }
    }
}