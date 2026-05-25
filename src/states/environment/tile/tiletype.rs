use crate::states::Tile;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum TileType {
    Empty,
    Solid(String), // String is the namespace and ID of the image
    Fluid,
}