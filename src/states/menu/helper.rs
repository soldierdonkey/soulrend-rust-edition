pub mod screen;
pub mod widget;
pub mod threepatch;
use serde::Deserialize;
use macroquad::prelude::Vec2;
use macroquad::prelude::Rect;
use crate::states::ArmorSlot;

#[derive(Debug, Deserialize, Clone)]
pub enum WindowType {
    Ui(UiSliceConfig),
    Background(String),
    None
}
#[derive(Debug, Deserialize, Clone)]
pub enum Centering {
    Center, // Do not use for widgets!
    Coordinates(f32, f32),
    Widget
}

#[derive(Debug, Deserialize, Clone)]
pub struct UiSliceConfig {
    pub sprite: String,
    pub centering: Centering,
    pub w: f32,
    pub h: f32,
}

impl UiSliceConfig {
    pub fn get_coordinates(&self) -> Vec2 {
        match self.centering {
            Centering::Center => {
                Vec2::new(crate::LOGICAL_WIDTH/2.0-self.w/2.0, crate::LOGICAL_HEIGHT/2.0-self.h/2.0)
            }
            Centering::Coordinates(x, y) => {
                Vec2::new(x, y)
            }
            _ => {
                panic!("Centering Style for UiSliceConfig with sprite: {} unsuccesfully matched: {:#?}", self.sprite, self.centering)
            }
        }
    }
    pub fn get_win_rect(&self) -> Rect {
        let (x, y) = self.get_coordinates().into();
        Rect::new(x, y , self.w, self.h)
    }
    pub fn get_rel_rect(&self, coordinates: Vec2) -> Rect {
        let (x, y) = self.get_coordinates().into();
        Rect::new(coordinates.x + x, coordinates.y + y, self.w, self.h)
    }
}

/// All possible actions a UI button can execute
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum UiAction {
    CloseCurrentMenu,
    EnterInstanceManager,
    OpenMenu(String),
}

/// Identifiers mapping UI slots straight to actual live player equipment states
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum SlotBinding {
    Armor(ArmorSlot),
    Hotbar(usize, usize),
    Inventory(usize, usize)
}