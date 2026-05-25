mod screen;
mod threepatch;
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
    Center,
    Coordinates(f32, f32)
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
}

#[derive(Debug, Deserialize, Clone)]
pub struct WidgetElement {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub kind: WidgetKind,
}

#[derive(Debug, Deserialize, Clone)]
pub enum WidgetKind {
    TextButton { 
        text: String, 
        action: UiAction,
        slice: UiSliceConfig,
        text_color: [u8; 4], // RGBA format
        font_size: f32
    },
    SpriteButton {
        action: UiAction,
        sprite: String,
    },
    InventorySlot { 
        binding: SlotBinding,
        sprite: String,
    },
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