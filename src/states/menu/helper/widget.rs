use serde::Deserialize;
use macroquad::prelude::*;
use super::*;
use crate::{Runtime, assets, states::threepatch::draw_3_patch_window};

#[derive(Debug, Deserialize, Clone)]
pub struct WidgetList {
    list: Vec<WidgetElement>
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
    Group {
        widgets: WidgetList,
        background_type: WindowType,
    }
}

impl Runtime {
    pub fn iter_widgets(&mut self,  window_center: Vec2, widget_list: &WidgetList) -> Option<UiAction> {
        let mut click_action = None;
        let mouse = self.mouse_position;
        let mouse_pressed = is_mouse_button_down(MouseButton::Left);
        let mouse_clicked = is_mouse_button_released(MouseButton::Left);
        for widget in &widget_list.list {
            // True Centering (The widget's own center aligns with the window center offset)
            let global_pos = window_center
                + vec2(widget.x, widget.y) 
                - vec2(widget.w / 2.0, widget.h / 2.0);
            let widget_rect = Rect::new(global_pos.x, global_pos.y, widget.w, widget.h);
            let is_hovered = widget_rect.contains(mouse);
            match &widget.kind {
                WidgetKind::TextButton { text, action, slice, text_color, font_size } => {
                    // Determine appropriate state asset skin on the fly!
                    let sprite_key = if is_hovered && mouse_pressed {
                        format!("{}/pressed", slice.sprite)
                    } else if is_hovered {
                        format!("{}/hover", slice.sprite)
                    } else {
                        format!("{}/normal", slice.sprite)
                    };
                    // Draw threepatch window
                    draw_3_patch_window(&sprite_key, Rect::new(global_pos.x, global_pos.y, widget.w, widget.h));

                    // Render Text centered inside the button bounds
                    let col = Color::from_rgba(text_color[0], text_color[1], text_color[2], text_color[3]);
                    // Basic vertical center alignment helper math
                    let text_size = measure_text(&text, None, font_size.clone() as u16, 1.0);
                    draw_text(&text, global_pos.x + slice.w/2.0 - text_size.width/2.0, global_pos.y + slice.h/2.0 + text_size.height/2.0, font_size.clone(), col);

                    if is_hovered && mouse_clicked {
                        click_action = Some(action.clone());
                    }
                }
                WidgetKind::SpriteButton { action, sprite } => {
                    // Determine appropriate state asset skin on the fly!
                    let sprite_key = if is_hovered && mouse_pressed {
                        format!("{}/pressed", sprite)
                    } else if is_hovered {
                        format!("{}/hover", sprite)
                    } else {
                        format!("{}/normal", sprite)
                    };

                    // Render dynamic texture button frame
                    if let Some(btn_tex) = assets::sprites::get(&sprite_key) {
                        draw_texture_ex(btn_tex, global_pos.x, global_pos.y, WHITE, DrawTextureParams {
                            dest_size: Some(vec2(widget.w, widget.h)),
                            ..Default::default()
                        });
                    }

                    if is_hovered && mouse_clicked {
                        click_action = Some(action.clone());
                    }
                }
                WidgetKind::InventorySlot { binding, sprite, } => {
                    let slot_tex_key = if is_hovered { format!("{}/hover", sprite) } else { format!("{}/normal", sprite) };

                    // Draw threepatch window
                    draw_3_patch_window(&slot_tex_key, Rect::new(global_pos.x, global_pos.y, crate::ITEM_SIZE, crate::ITEM_SIZE));
                }
                WidgetKind::Group { widgets, background_type} => {
                    // Replace with pub fn or trait on WindowType
                    match background_type {
                        WindowType::Ui(window) => {
                            draw_3_patch_window(&window.sprite, window.get_win_rect());
                        },
                        WindowType::Background(background) => {
                            match assets::sprites::get(background) {
                                Some(texture) => {
                                    draw_texture_ex(
                                        texture,
                                        0.0,
                                        0.0,
                                        WHITE,
                                        DrawTextureParams {
                                            dest_size: Some(vec2(crate::VIRTUAL_WIDTH, crate::VIRTUAL_HEIGHT)),
                                            ..Default::default()
                                        },
                                    );
                                },
                                None => {
                                    panic!("Sprite with ID '{}' not found in Sprite Registry!", background);
                                }
                            }
                        },
                        WindowType::None => {}
                    }
                    click_action = self.iter_widgets(vec2(global_pos.x+widget.w/2.0, global_pos.y+widget.h/2.0), widgets)
                }
            }
        }
        click_action
    }
}