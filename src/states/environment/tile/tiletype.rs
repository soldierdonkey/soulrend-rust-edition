use crate::states::Tile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Solid(String), // String is the namespace and ID of the image
    Fluid,
}