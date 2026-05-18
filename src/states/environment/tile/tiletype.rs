use crate::states::Tile;

#[derive(Clone)]
pub enum TileType {
    Empty,
    Solid(String), // String is the namespace and ID of the image
    Fluid,
}