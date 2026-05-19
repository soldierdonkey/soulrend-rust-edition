use crate::states::Tile;

#[derive(Debug, Clone)]
pub enum TileType {
    Empty,
    Solid(String), // String is the namespace and ID of the image
    Fluid,
}