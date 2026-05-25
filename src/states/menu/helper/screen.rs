use crate::assets;
use crate::states::*;
use crate::runtime::Runtime;
use crate::states::menu::helper::threepatch::draw_3_patch_window;
use crate::states::{Player, UiAction, WidgetKind, WindowType};
use macroquad::prelude::*;
use macroquad::math::Vec2;

impl Runtime {
    pub fn render_screen(&mut self, screen_id: String) -> Option<UiAction> {
        let mut click_action = None;
        let mouse = self.mouse_position;
        let mouse_pressed = is_mouse_button_down(MouseButton::Left);
        let mouse_clicked = is_mouse_button_released(MouseButton::Left);
        let screen = match assets::uiscreen::get(&screen_id) {
            Some(screen_data) => {
                screen_data
            }
            None => {
                panic!("Error! UiScreen Type: {} is not in the UIScreen Registry!", screen_id);
            }
        };
        let window_center: Vec2 = match &screen.window {
            WindowType::Ui(window) => {
                // 1. Draw the data-driven window skin via 3-Patch Rotation
                // Use screen_pos as the dynamic anchor override
                draw_3_patch_window(&window.sprite, window.get_win_rect());

                // Calculate center using screen_pos instead of screen.window.x/y
                window.get_coordinates() + vec2(window.w / 2.0, window.h / 2.0)
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
                vec2(crate::VIRTUAL_WIDTH / 2.0, crate::VIRTUAL_HEIGHT / 2.0)
            },
            WindowType::None => {
               vec2(crate::VIRTUAL_WIDTH / 2.0, crate::VIRTUAL_HEIGHT / 2.0) 
            }
        };

        // 2. Loop and draw widgets based on center-relative JSON configurations
        for widget in &screen.widgets {
            
            // CHOICE A: True Centering (The widget's own center aligns with the window center offset)
            let global_pos = window_center 
                + vec2(widget.x, widget.y) 
                - vec2(widget.w / 2.0, widget.h / 2.0);

            // CHOICE B: Top-Left Centering (Comment out Choice A and uncomment this if you prefer it)
            // let global_pos = window_center + vec2(widget.x, widget.y);

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

                    // Render dynamic texture button frame
                    if let Some(btn_tex) = assets::sprites::get(&sprite_key) {
                        // draw_3_patch_window(slice.sprite, dest_rect, color, scale, debug_mode);
                        draw_texture_ex(btn_tex, global_pos.x, global_pos.y, WHITE, DrawTextureParams {
                            dest_size: Some(vec2(widget.w, widget.h)),
                            ..Default::default()
                        });
                    }

                    // Render Text centered inside the button bounds
                    let col = Color::from_rgba(text_color[0], text_color[1], text_color[2], text_color[3]);
                    // Basic vertical center alignment helper math
                    let text_size = measure_text(text, None, *font_size as u16, 1.0);
                    draw_text(&text, global_pos.x + widget.w/2.0 - text_size.width/2.0, global_pos.y + widget.h/2.0 + text_size.height/2.0, *font_size, col);

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

                    // Draw configured slot background grid square
                    if let Some(slot_bg) = assets::sprites::get(&slot_tex_key) {
                        draw_texture_ex(slot_bg, global_pos.x, global_pos.y, WHITE, DrawTextureParams {
                            dest_size: Some(vec2(widget.w, widget.h)),
                            ..Default::default()
                        });
                    }
                }
            }
        }

        self.process_ui_action(&click_action);
        click_action
    }
    pub fn process_ui_action(&mut self, input: &Option<UiAction>) {
        match input {
            Some(action) => {
                match action {
                    UiAction::EnterInstanceManager => {
                        self.current_state = GameState::InstanceManager;
                    },
                    _ => {
                        panic!("Uncovered Ui Action: {:#?}!", action)
                    }
                }
            }
            None => {}
        }
    }
}