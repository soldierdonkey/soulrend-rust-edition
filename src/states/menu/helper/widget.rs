use serde::Deserialize;
use macroquad::prelude::*;
use super::*;
use crate::{Runtime, assets, runtime::get_mouse_position, states::{GameState::InGame, SlotBinding, UiAction::SwitchItemWithMouse, threepatch::draw_3_patch_window}};

#[derive(Debug, Deserialize, Clone)]
pub struct WidgetList {
    list: Vec<WidgetElement>
}

#[derive(Debug, Deserialize, Clone)]
pub struct WidgetElement {
    pub position: Centering,
    pub size: (f32, f32),
    pub kind: WidgetKind,
}

#[derive(Debug, Deserialize, Clone)]
pub enum WidgetKind {
    TextButton { 
        text: String, 
        action: UiAction,
        slice: Option<String>,
        text_color: [u8; 4], // RGBA format
        font_size: f32
    },
    SpriteButton {
        action: UiAction,
        sprite: String,
    },
    InventorySlot { 
        binding: SlotBinding,
        sprite: Option<String>,
        disabled: Option<bool>
    },
    Group {
        widgets: WidgetList,
        background_type: WindowType,
    }
}

impl Runtime {
    pub fn iter_widgets(&mut self,  window_center: Vec2, widget_list: &WidgetList) -> Vec<UiAction> {
        let mut click_action: Vec<UiAction> = vec![];
        let mouse = get_mouse_position();
        let mouse_pressed = is_mouse_button_down(MouseButton::Left);
        let mouse_clicked = is_mouse_button_released(MouseButton::Left);
        for widget in &widget_list.list {
            // True Centering (The widget's own center aligns with the window center offset)
            let mut global_pos = centering_to_coordinates(&widget.position, Vec2::new(widget.size.0, widget.size.1), window_center);
            let widget_rect = Rect::new(global_pos.x, global_pos.y, widget.size.0, widget.size.1);
            let is_hovered = widget_rect.contains(mouse);
            match &widget.kind {
                WidgetKind::TextButton { text, action, slice, text_color, font_size } => {
                    // Determine appropriate state asset skin on the fly!
                    if let Some(sprite) = slice {
                        let sprite_key = if is_hovered && mouse_pressed {
                            format!("{}/pressed", sprite)
                        } else if is_hovered {
                            format!("{}/hover", sprite)
                        } else {
                            format!("{}/normal", sprite)
                        };
                        // Draw threepatch window
                        draw_3_patch_window(&sprite_key, Rect::new(global_pos.x, global_pos.y, widget.size.0, widget.size.1));
                    }

                    // Render Text centered inside the button bounds
                    let col = Color::from_rgba(text_color[0], text_color[1], text_color[2], text_color[3]);
                    // Basic vertical center alignment helper math
                    let text_size = measure_text(&text, None, font_size.clone() as u16, 1.0);
                    draw_text(&text, global_pos.x + widget.size.0/2.0 - text_size.width/2.0, global_pos.y + widget.size.1/2.0 + text_size.height/2.0, font_size.clone(), col);

                    if is_hovered && mouse_clicked {
                        click_action.push(action.clone());
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
                            dest_size: Some(vec2(widget.size.0, widget.size.1)),
                            ..Default::default()
                        });
                    }

                    if is_hovered && mouse_clicked {
                        click_action.push(action.clone());
                    }
                }
                WidgetKind::InventorySlot { binding, sprite, disabled} => {
                    if let Some(sprite) = sprite {
                        let slot_tex_key = if is_hovered { format!("{}/hover", sprite) } else { format!("{}/normal", sprite) };

                        // Draw threepatch window
                        draw_3_patch_window(&slot_tex_key, Rect::new(global_pos.x, global_pos.y, crate::ITEM_SIZE, crate::ITEM_SIZE));
                    }
                    if let InGame(in_game_state) = &self.current_state {
                        if let Some(item) = in_game_state.player.inventory.read_slot(binding) {
                            if let Some(item_entry) = assets::items::get(&item.id) {
                                if let Some(texture) = assets::sprites::get(&item_entry.sprite) {
                                    draw_texture_ex(texture, global_pos.x, global_pos.y, WHITE, DrawTextureParams {
                                        dest_size: Some(vec2(widget.size.0, widget.size.1)),
                                        ..Default::default()
                                    });
                                } else {
                                    crate::global_panic!(asset sprites &item_entry.sprite)
                                }
                            } else {
                                crate::global_panic!(data items &item.id)
                            }
                        }
                        // There is no item, no worries
                    } else {
                        crate::global_panic!(gamestate widget)
                    }
                    if let Some(true) = disabled {

                    } else {
                        if is_hovered && mouse_clicked {
                            println!("Item Switch Request Registered with binding: {:?}", binding.clone());
                            click_action.push(UiAction::SwitchItemWithMouse(binding.clone()));
                        }
                    }
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
                    let mut new_actions: Vec<UiAction> = self.iter_widgets(vec2(global_pos.x+widget.size.0/2.0, global_pos.y+widget.size.1/2.0), widgets);
                    click_action.extend(new_actions);
                }
            }
        }
        click_action
    }
}