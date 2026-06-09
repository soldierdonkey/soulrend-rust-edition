use crate::assets;
use crate::runtime::get_mouse_position;
use crate::states::GameState::InGame;
use crate::states::*;
use crate::runtime::Runtime;
use crate::states::menu::helper::threepatch::draw_3_patch_window;
use crate::states::{Player, UiAction, WindowType};
use super::widget::*;
use macroquad::prelude::*;
use macroquad::math::Vec2;

impl Runtime {
    pub fn render_screen(&mut self, screen_id: String) -> Vec<UiAction> {
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
        let click_action = self.iter_widgets(window_center, &screen.widgets);
        self.process_ui_action(&click_action);
        click_action
    }
    pub fn process_ui_action(&mut self, input: &Vec<UiAction>) {
        input.iter().for_each(
            |action|
            match action {
                UiAction::EnterInstanceManager => {
                    self.current_state = GameState::InstanceManager;
                },
                UiAction::SwitchItemWithMouse(slot) => {
                    println!("Item Switch Request Caught with binding: {:?}", slot);
                    if let InGame(in_game_state) = &mut self.current_state {
                        println!("Item Switch Begun with binding: {:?}", slot);
                        in_game_state.player.inventory.swap_slots_safe((slot, &SlotBinding::Mouse));
                    } else {
                        crate::global_panic!(gamestate action)
                    }
                }
                _ => {
                    crate::global_panic!(uiaction action);
                }
            }
        );
    }
}