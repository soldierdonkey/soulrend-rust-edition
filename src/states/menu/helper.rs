pub mod screen;
pub mod widget;
pub mod threepatch;
use macroquad::math::vec2;
use serde::Deserialize;
use macroquad::prelude::Vec2;
use macroquad::prelude::Rect;
use crate::runtime::get_mouse_position;
use crate::states::ArmorSlot;
use crate::states::Item;
use crate::states::SlotBinding;

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
    Mouse
}

pub fn centering_to_coordinates(centering: &Centering, size: Vec2, position: Vec2) -> Vec2 {
    match centering {
        Centering::Center => {
            Vec2::new(crate::LOGICAL_WIDTH/2.0-size.x/2.0, crate::LOGICAL_HEIGHT/2.0-size.y/2.0)
        }
        Centering::Coordinates(x, y) => {
            Vec2::new(*x, *y)-size/2.0+position
        }
        Centering::Mouse => {
            get_mouse_position()-size/2.0
        }
    }
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
        centering_to_coordinates(&self.centering, Vec2::new(self.w, self.h), Vec2::new(crate::LOGICAL_WIDTH/2.0, crate::LOGICAL_HEIGHT/2.0))
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
#[derive(Debug, Deserialize, Clone)]
pub enum UiAction {
    CloseCurrentMenu,
    EnterInstanceManager,
    SwitchItemWithMouse(SlotBinding),
    OpenMenu(String),
    Tooltip(SlotBinding),
}