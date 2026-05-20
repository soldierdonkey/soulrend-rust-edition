// Import file types
use super::super::tile::Tile;
use super::super::tile::TileType;
use crate::states::environment::terrain::hills::*;

pub struct Scene {
    pub scene_id: String,
    pub name: String,
    pub description: String,
    pub starts: Vec<(u32, u32)>,
    pub tiles: SceneMap,
}
impl Scene {
    pub fn new(scene_id: String, name: String, description: String, size: (u32, u32)) -> Self {
        Self {
            scene_id,
            name,
            description,
            starts: Vec::new(),
            tiles: generate_hills(size.0 as usize, size.1 as usize, 3),
        }
    }
    pub fn add_start(&mut self, position: (u32, u32)) {
        self.starts.push(position);
    }
}
#[derive(Debug)]
pub struct SceneMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<Tile>>,
}
impl SceneMap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![Tile::new(TileType::Empty); width as usize]; height as usize],
        }
    }
}