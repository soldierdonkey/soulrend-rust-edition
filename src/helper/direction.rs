use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}