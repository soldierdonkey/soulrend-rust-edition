#[derive(Clone, Debug)]
pub struct TileData {
    pub trapped: bool,
}
impl TileData {
    pub fn new() -> Self {
        Self {
            trapped: false,
        }
    }
}