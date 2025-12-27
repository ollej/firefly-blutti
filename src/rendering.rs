use firefly_rust::{Color, Point, WIDTH, clear_screen, draw_image};
use fixedstr::{str_format, str32};

use crate::{constants::*, drawable::*, drawing::*, state::*};

pub fn render_title() {
    let state = get_state();
    draw_image(&state.title.as_image(), Point { x: 0, y: 0 });
    display_centered_message(Some(Color::White), &["Press (E) to start!"]);
}

pub fn render_died() {
    let state = get_state();
    state.level.draw();
    state.blutti.draw();
    render_ui();
    display_centered_message(None, &["You died!", "Press (E) to restart level"]);
}

pub fn render_gameover(won: bool) {
    let state = get_state();
    state.level.draw();
    state.blutti.draw();
    render_ui();
    if won {
        display_centered_message(None, &["You win!", "Press (E) to start next level!"]);
    } else {
        display_centered_message(None, &["Game Over!", "Press (E) to start again!"]);
    }
}

pub fn render_ui() {
    let state = get_state();
    display_text(
        str_format!(str32, "Points: {}", state.blutti.points).as_str(),
        Point {
            x: 4,
            y: FONT_BASE_LINE + 4,
        },
    );
    for heart in 0..state.blutti.lives {
        draw_tile(
            11,
            Point {
                x: WIDTH - heart * TILE_WIDTH - TILE_WIDTH - 3,
                y: 4,
            },
        );
    }
}

pub fn render_credits() {
    clear_screen(Color::White);
    display_left_message(&CREDITS);
}

pub fn render_info() {
    clear_screen(Color::White);
    display_left_message(&INFO);
}

pub fn render_playing() {
    let state = get_state();

    state.level.draw();
    state.blutti.draw();
    state.level.draw_children();
    render_ui();
}
