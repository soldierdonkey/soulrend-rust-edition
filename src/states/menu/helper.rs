mod screen;
mod threepatch;
use serde::Deserialize;

use crate::states::ArmorSlot;

#[derive(Debug, Deserialize, Clone)]
pub enum WindowType {
    Ui(UiSliceConfig),
    Background(String)
}

#[derive(Debug, Deserialize, Clone)]
pub struct UiSliceConfig {
    pub sprite: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
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
        sprite: String,
        text_color: [u8; 4], // RGBA format
        font_size: f32
    },
    SpriteButton {
        action: UiAction,
        sprite: String,
    },
    InventorySlot { 
        binding: SlotBinding,
        background_sprite: String,
        hover_sprite: String,
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