use firefly_rust::{draw_sub_image, draw_text, Color, Point, Size, HEIGHT, WIDTH};

use crate::{constants::*, state::*};

pub type Sprite = i32;

pub fn get_origin_point_of_position(position: Point) -> Point {
    Point {
        x: position.x / TILE_WIDTH * TILE_WIDTH,
        y: position.y / TILE_HEIGHT * TILE_HEIGHT,
    }
}

pub fn get_tile_index(point: Point) -> i32 {
    let tile_x = point.x / TILE_WIDTH;
    let tile_y = point.y / TILE_WIDTH;
    //log_debug(str_format!(str256, "tile_x: {} tile_y: {}", tile_x, tile_y).as_str());
    tile_y * TILES_H + tile_x
}

pub fn draw_tile(sprite: Sprite, point: Point) {
    let state = get_state();
    let tile_sprite = state.spritesheet.as_image().sub(
        Point {
            x: ((sprite % SPRITES_H) * TILE_WIDTH),
            y: ((sprite / SPRITES_H) * TILE_HEIGHT),
        },
        Size {
            width: 8,
            height: 8,
        },
    );
    draw_sub_image(&tile_sprite, point);
}

pub fn display_text(text: &str, position: Point) {
    let state = get_state();
    let color = state.level.font_color;
    display_text_color(text, position, color);
}

pub fn display_text_color(text: &str, position: Point, color: Color) {
    let state = get_state();
    let font = state.font.as_font();
    draw_text(text, &font, position, color);
}

pub fn display_centered_message(color: Option<Color>, lines: &[&str]) {
    let state = get_state();
    let color = color.unwrap_or(state.level.font_color);
    let y_pos: i32 = HEIGHT / 2 + FONT_BASE_LINE - lines.len() as i32 * LINE_HEIGHT / 2;
    for (i, line) in lines.iter().enumerate() {
        display_text_color(
            line,
            Point {
                x: WIDTH / 2 - (line.len() as i32 * HALF_FONT_WIDTH),
                y: y_pos + i as i32 * LINE_HEIGHT,
            },
            color,
        );
    }
}

pub fn display_left_message(lines: &[&str]) {
    let y_pos: i32 = FONT_BASE_LINE + 4;
    for (i, line) in lines.iter().enumerate() {
        display_text_color(
            line,
            Point {
                x: 4,
                y: y_pos + i as i32 * LINE_HEIGHT,
            },
            Color::Black,
        );
    }
}
