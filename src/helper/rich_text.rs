use macroquad::prelude::*;
use crate::states::threepatch::draw_3_patch_window;
use crate::{assets::sprite_modularity::Palette};
use crate::assets;

// -------------------------------------------------------------
// Core Data Structures
// -------------------------------------------------------------

enum RichChunk {
    Text {
        text: String,
        color: Color,
        strikethrough: bool,
        underline: bool,
        width: f32,
    },
    HorizontalRule {
        color: Color,
        width: f32,
    },
}

struct LayoutLine {
    chunks: Vec<RichChunk>,
    is_centered: bool,
    width: f32,
}

// -------------------------------------------------------------
// Helper Tokenizer Engine
// -------------------------------------------------------------

fn tokenize_text(text: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut start = 0;
    let mut in_whitespace = false;
    for (idx, ch) in text.char_indices() {
        let is_ws = ch.is_whitespace();
        if idx == 0 { in_whitespace = is_ws; }
        else if is_ws != in_whitespace {
            tokens.push(&text[start..idx]);
            start = idx;
            in_whitespace = is_ws;
        }
    }
    if start < text.len() { tokens.push(&text[start..]); }
    tokens
}

// -------------------------------------------------------------
// Rich Text Processing & Rendering Pipeline
// -------------------------------------------------------------

pub fn draw_rich_text(
    lines: &[impl AsRef<str>],
    x: f32,
    y: f32,
    max_width: f32,
    threepatch_id: &Option<String>,
    palette: &Palette,
) {
    let font = assets::fonts::get().unwrap_or_else(|| crate::global_panic!(font));
    let default_color = WHITE;
    
    // 1. Calculate Padding Upfront
    let mut padding = 0.0;
    if let Some(tp_id) = threepatch_id {
        padding = assets::threepatch::get(&tp_id.to_string())
            .unwrap_or_else(|| crate::global_panic!(data threepatch &tp_id.to_string())).padding;
    }
    
    // The text drawing starts inside the padding offset
    let draw_x_start = x + padding;
    let draw_y_start = y + padding;

    let line_height = crate::FONT_SIZE;
    let mut all_layout_lines: Vec<LayoutLine> = Vec::new();

    // ---------------------------------------------------------
    // STAGE 1: Compilation & Layout
    // ---------------------------------------------------------
    for line_ref in lines {
        let line = line_ref.as_ref();
        let parts: Vec<&str> = line.split('§').collect();
        let mut active_color = default_color;
        let mut strikethrough = false;
        let mut underline = false;
        let mut is_centered = false;
        let mut current_chunks = Vec::new();
        let mut current_line_width = 0.0;

        for (index, part) in parts.iter().enumerate() {
            if index % 2 == 1 {
                let cmd = part.trim();
                match cmd {
                    "r" => { active_color = default_color; strikethrough = false; underline = false; }
                    "strikethrough" => strikethrough = true,
                    "underline" => underline = true,
                    "centered" => is_centered = true,
                    _ if cmd.starts_with("color:") => {
                        let color_key = &cmd["color:".len()..];
                        let color_id = palette.colors.get(color_key).unwrap_or_else(|| crate::global_panic!(palette color_key => palette));
                        active_color = assets::hexcolor::get(color_id).unwrap_or_else(|| crate::global_panic!(data hexcolor color_id)).to_color();
                    }
                    _ if cmd.starts_with("hr:") => {
                        let color_key = &cmd["hr:".len()..];
                        let color_id = palette.colors.get(color_key).unwrap_or_else(|| crate::global_panic!(palette color_key => palette));
                        let hr_color = assets::hexcolor::get(color_id).unwrap_or_else(|| crate::global_panic!(data hexcolor color_id)).to_color();
                        let hr_width = if current_chunks.is_empty() { max_width.min(400.0) } else { 100.0 };
                        if current_line_width + hr_width > max_width && current_line_width > 0.0 {
                            all_layout_lines.push(LayoutLine { chunks: current_chunks, is_centered, width: current_line_width });
                            current_chunks = Vec::new(); current_line_width = 0.0;
                        }
                        current_chunks.push(RichChunk::HorizontalRule { color: hr_color, width: hr_width });
                        current_line_width += hr_width;
                    }
                    _ => {}
                }
            } else if !part.is_empty() {
                for token in tokenize_text(part) {
                    let is_ws = token.chars().next().map_or(false, |c| c.is_whitespace());
                    let dims = measure_text(token, Some(&font), crate::FONT_SIZE as u16, 1.0);
                    if current_line_width + dims.width > max_width && current_line_width > 0.0 {
                        all_layout_lines.push(LayoutLine { chunks: current_chunks, is_centered, width: current_line_width });
                        current_chunks = Vec::new(); current_line_width = 0.0;
                        if is_ws { continue; }
                    }
                    current_chunks.push(RichChunk::Text { text: token.to_string(), color: active_color, strikethrough, underline, width: dims.width });
                    current_line_width += dims.width;
                }
            }
        }
        if !current_chunks.is_empty() || all_layout_lines.is_empty() {
            all_layout_lines.push(LayoutLine { chunks: current_chunks, is_centered, width: current_line_width });
        }
    }

    // ---------------------------------------------------------
    // STAGE 2: Bounds & Background
    // ---------------------------------------------------------
    let actual_width = all_layout_lines.iter().map(|l| l.width).fold(0.0, f32::max).min(max_width);
    let total_height = all_layout_lines.len() as f32 * line_height;

    if let Some(tp_id) = threepatch_id {
        // Draw the background window at x, y, wrapping the text + padding
        draw_3_patch_window(&tp_id.to_string(), Rect::new(x, y, actual_width + padding * 2.0, total_height + padding * 2.0));
    }

    // ---------------------------------------------------------
    // STAGE 3: Final Render Pass
    // ---------------------------------------------------------
    let ascent = measure_text("M", Some(&font), crate::FONT_SIZE as u16, 1.0).offset_y;

    for (i, layout_line) in all_layout_lines.into_iter().enumerate() {
        // Start drawing text at the offset draw_y_start
        let current_y = draw_y_start + (i as f32 * line_height) + ascent;
        
        let mut current_x = if layout_line.is_centered {
            draw_x_start + (actual_width - layout_line.width) / 2.0 
        } else {
            draw_x_start
        };

        for chunk in layout_line.chunks {
            match chunk {
                RichChunk::Text { text, color, strikethrough, underline, width: chunk_width } => {
                    draw_text_ex(&text, current_x, current_y, TextParams {
                        font: Some(&font),
                        font_size: crate::FONT_SIZE as u16,
                        font_scale: 1.0,
                        color,
                        ..Default::default()
                    });
                    
                    if underline { draw_line(current_x, current_y + 4.0, current_x + chunk_width, current_y + 4.0, 2.0, color); }
                    if strikethrough { draw_line(current_x, current_y - (ascent * 0.5), current_x + chunk_width, current_y - (ascent * 0.5), 2.0, color); }
                    current_x += chunk_width;
                }
                RichChunk::HorizontalRule { color, width: hr_width } => {
                    let rule_y = current_y - (ascent * 0.5);
                    draw_line(current_x, rule_y, current_x + hr_width.min(actual_width), rule_y, 4.0, color);
                    current_x += hr_width;
                }
            }
        }
    }
}