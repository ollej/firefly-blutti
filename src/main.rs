#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;

mod animation;
mod blutti;
mod collision;
mod constants;
mod direction;
mod drawable;
mod drawing;
mod functions;
mod game_state;
mod level;
mod monster;
mod particle;
mod player_state;
mod point_math;
mod rect;
mod rendering;
mod serde;
mod state;
mod tile_collider;
mod updateable;
mod vec2;

use functions::*;
use game_state::*;
use level::*;
use rendering::*;
use state::*;

use firefly_rust::*;

#[unsafe(no_mangle)]
extern "C" fn cheat(cmd: i32, val: i32) -> i32 {
    let state = get_state();
    match cmd {
        1 => Level::restart(val, true),
        2 => {
            for blutti in state.bluttis.iter_mut() {
                if blutti.peer == get_me() {
                    blutti.add_lives(val);
                }
            }
            val
        }
        3 => {
            for blutti in state.bluttis.iter_mut() {
                if blutti.peer == get_me() {
                    blutti.add_points(val);
                }
            }
            val
        }
        4 => {
            for blutti in state.bluttis.iter_mut() {
                if blutti.peer == get_me() {
                    blutti.die();
                }
            }
            1
        }
        5 => {
            for blutti in state.bluttis.iter_mut() {
                if blutti.peer == get_me() {
                    blutti.iddqd = val > 0
                }
            }
            1
        }
        _ => 0,
    }
}

#[unsafe(no_mangle)]
extern "C" fn handle_menu(menu_item: u8) {
    let state = get_state();
    match menu_item {
        1 => state.game_state = GameState::Credits,
        2 => {
            Level::restart(1, false);
        }
        3 => state.game_state = GameState::Info,
        _ => (),
    }
}

#[unsafe(no_mangle)]
extern "C" fn boot() {
    let fx = audio::OUT.add_gain(1.0);
    let theme = audio::OUT.add_gain(0.5);
    let level = Level::load_level(1);
    let state = State {
        bluttis: Vec::new(),
        spritesheet: load_file_buf("spritesheet").unwrap().into(),
        title: load_file_buf("_splash").unwrap().into(),
        font: load_file_buf("font").unwrap().into(),
        fx,
        theme,
        level,
        game_state: GameState::Title,
        buttons: Buttons::default(),
    };
    #[allow(static_mut_refs)]
    unsafe { STATE.set(state) }.ok().unwrap();
    add_menu_item(1, "Credits");
    add_menu_item(2, "Restart");
    add_menu_item(3, "Info");
    play_music("sound_theme");
    set_seed(4711);
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    let state = get_state();
    let buttons = read_buttons(Peer::COMBINED);
    let just_pressed = buttons.just_pressed(&state.buttons);
    state.buttons = buttons;

    match state.game_state {
        GameState::Title => {
            if just_pressed.any() {
                state.start_game();
            }
        }
        GameState::Credits => {
            if just_pressed.any() {
                state.game_state = GameState::Title;
            }
        }
        GameState::Info => {
            if just_pressed.any() {
                state.game_state = GameState::Title;
            }
        }
        GameState::Died => {
            state.update_animation();
            if just_pressed.any() {
                state.reset();
                state.game_state = GameState::Playing;
            }
        }
        GameState::Playing => {
            state.update();
        }
        GameState::GameOver(won) => {
            state.update_animation();
            if just_pressed.e {
                state.restart(won);
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    let state = get_state();
    match state.game_state {
        GameState::Title => render_title(),
        GameState::Credits => render_credits(),
        GameState::Info => render_info(),
        GameState::Playing => render_playing(),
        GameState::Died => render_died(),
        GameState::GameOver(won) => render_gameover(won),
    }
}
