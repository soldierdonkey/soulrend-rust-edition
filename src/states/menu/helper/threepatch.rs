use macroquad::prelude::*;
use crate::assets;
use std::f32::consts::{FRAC_PI_2, PI};

pub fn draw_3_patch_window(
    id: &String,
    dest_rect: Rect,
) {
    let color = WHITE;
    let corner_key = format!("{}/corner", id);
    let side_key = format!("{}/side", id);
    let center_key = format!("{}/center", id);
    let (corner, side, center) = if let (Some(c_tex), Some(s_tex), Some(ctr_tex)) = (
        assets::sprites::get(&corner_key),
        assets::sprites::get(&side_key),
        assets::sprites::get(&center_key),
    ) {
        (c_tex, s_tex, ctr_tex)
    } else {
        panic!("Error! Any one of Sprites: {}, {}, {} is not in the Sprite Registry!", &corner_key, &side_key, &center_key);
    };
    let x = dest_rect.x;
    let y = dest_rect.y;
    let w = dest_rect.w;
    let h = dest_rect.h;

    // Apply the scaling factor to the raw texture dimensions
    let (c_size, s_thick) = if let Some(size) = assets::threepatch::get(id) {
        (size.border_size, size.border_size)
    } else {
        (corner.width(), side.height())
    };

    // The inner fill area shrinks because the borders take up more space
    let dynamic_w = w - (c_size * 2.0);
    let dynamic_h = h - (c_size * 2.0);

    // if debug_mode {
    //     println!("--- 3-Patch Scaled Telemetry ---");
    //     println!("Scale Factor: {}x, {}x", c_size, s_thick);
    //     println!("Scaled Corner Size: {}, Scaled Side Thickness: {}", c_size, s_thick);
    //     println!("Dynamic Fill Workspace: {}x{}", dynamic_w, dynamic_h);
    // }

    // =========================================================================
    // 1. DRAW THE CENTER
    // =========================================================================
    if dynamic_w > 0.0 && dynamic_h > 0.0 {
        draw_texture_ex(
            center,
            x + c_size,
            y + c_size,
            color,
            DrawTextureParams {
                dest_size: Some(vec2(dynamic_w, dynamic_h)),
                ..Default::default()
            },
        );
    }

    // =========================================================================
    // 2. DRAW THE CORNERS (Using scaled dest_size)
    // =========================================================================
    
    // Top-Left (0°)
    draw_texture_ex(corner, x, y, color, DrawTextureParams {
        dest_size: Some(vec2(c_size, c_size)),
        ..Default::default()
    });
    
    // Top-Right (90°)
    draw_texture_ex(corner, x + w, y, color, DrawTextureParams {
        dest_size: Some(vec2(c_size, c_size)),
        rotation: FRAC_PI_2,
        pivot: Some(vec2(x + w, y)),
        ..Default::default()
    });
    
    // Bottom-Right (180°)
    draw_texture_ex(corner, x + w, y + h, color, DrawTextureParams {
        dest_size: Some(vec2(c_size, c_size)),
        rotation: PI,
        pivot: Some(vec2(x + w, y + h)),
        ..Default::default()
    });
    
    // Bottom-Left (270°)
    draw_texture_ex(corner, x, y + h, color, DrawTextureParams {
        dest_size: Some(vec2(c_size, c_size)),
        rotation: FRAC_PI_2 * 3.0,
        pivot: Some(vec2(x, y + h)),
        ..Default::default()
    });

    // =========================================================================
    // 3. DRAW THE SIDES (Using scaled side thicknesses)
    // =========================================================================
    if dynamic_w > 0.0 {
        // Top Side (0°)
        draw_texture_ex(
            side,
            x + c_size,
            y,
            color,
            DrawTextureParams {
                dest_size: Some(vec2(dynamic_w, s_thick)),
                ..Default::default()
            },
        );

        // Bottom Side (180°)
        let b_x = x + w - c_size;
        let b_y = y + h;
        draw_texture_ex(
            side,
            b_x,
            b_y,
            color,
            DrawTextureParams {
                dest_size: Some(vec2(dynamic_w, s_thick)),
                rotation: PI,
                pivot: Some(vec2(b_x, b_y)),
                ..Default::default()
            },
        );
    }

    if dynamic_h > 0.0 {
        // Right Side (90°)
        let r_x = x + w;
        let r_y = y + c_size;
        draw_texture_ex(
            side,
            r_x,
            r_y,
            color,
            DrawTextureParams {
                dest_size: Some(vec2(dynamic_h, s_thick)),
                rotation: FRAC_PI_2,
                pivot: Some(vec2(r_x, r_y)),
                ..Default::default()
            },
        );

        // Left Side (270°)
        let l_x = x;
        let l_y = y + h - c_size;
        draw_texture_ex(
            side,
            l_x,
            l_y,
            color,
            DrawTextureParams {
                dest_size: Some(vec2(dynamic_h, s_thick)),
                rotation: FRAC_PI_2 * 3.0,
                pivot: Some(vec2(l_x, l_y)),
                ..Default::default()
            },
        );
    }
}