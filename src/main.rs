#![no_std]
#![no_main]

extern crate alloc;

use firefly_rust::*;

use firefly_blutti::{
    blutti::*, functions::*, game_state::*, level::*, rendering::*, state::*, updateable::*,
};

#[unsafe(no_mangle)]
extern "C" fn cheat(cmd: i32, val: i32) -> i32 {
    let state = get_state();
    match cmd {
        1 => Level::restart(val, true),
        2 => state.blutti.add_lives(val),
        3 => state.blutti.add_points(val),
        4 => {
            state.blutti.die();
            1
        }
        5 => {
            state.blutti.iddqd = val > 0;
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
        blutti: Blutti::with_start_position(level.start_position),
        spritesheet: load_file_buf("spritesheet").unwrap(),
        title: load_file_buf("_splash").unwrap(),
        font: load_file_buf("font").unwrap(),
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
    let just_released = buttons.just_released(&state.buttons);
    state.buttons = buttons;

    match state.game_state {
        GameState::Title => {
            if just_pressed.any() {
                state.game_state = GameState::Playing;
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
            state.blutti.animation.update();
            if just_pressed.any() {
                state.blutti.reset();
                state.game_state = GameState::Playing;
            }
        }
        GameState::Playing => {
            let pad = read_pad(Peer::COMBINED);
            if let Some(pad) = pad {
                if pad.y > 100 {
                    state.blutti.move_up(axis_to_speed(pad.y));
                } else if pad.y < -100 {
                    state.blutti.move_down(axis_to_speed(pad.y));
                }
                if pad.x < -100 {
                    state.blutti.move_left(axis_to_speed(pad.x));
                } else if pad.x > 100 {
                    state.blutti.move_right(axis_to_speed(pad.x));
                }
            } else {
                state.blutti.stop();
            }
            if just_pressed.s {
                state.blutti.start_jump();
            }
            if just_released.s {
                state.blutti.stop_jump();
            }
            if just_pressed.w {
                state.blutti.start_dash();
            }
            if just_pressed.e {
                state.blutti.toggle_debug();
            }
            state.level.update();
            state.blutti.update();
            state.blutti.handle_effects();

            if !state.blutti.is_alive() || state.blutti.finished_level {
                state.game_state = GameState::GameOver(state.blutti.finished_level);
            } else if state.blutti.died {
                state.game_state = GameState::Died;
            }
        }
        GameState::GameOver(won) => {
            state.blutti.animation.update();
            if just_pressed.e {
                if won {
                    Level::restart(state.blutti.current_level + 1, won);
                } else {
                    Level::restart(1, won);
                }
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
