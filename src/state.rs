use alloc::vec::Vec;
use core::cell::OnceCell;
use firefly_rust::*;

use crate::{blutti::*, functions::*, game_state::*, level::*, updateable::*};

pub static mut STATE: OnceCell<State> = OnceCell::new();

pub fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe { STATE.get_mut() }.unwrap()
}

pub struct State {
    pub bluttis: Vec<Blutti>,
    pub spritesheet: ImageBuf,
    pub title: ImageBuf,
    pub font: FontBuf,
    pub fx: audio::Node<audio::Gain>,
    pub theme: audio::Node<audio::Gain>,
    pub level: Level,
    pub game_state: GameState,
    pub buttons: Buttons,
}

impl State {
    pub fn update(&mut self) {
        self.level.update();

        for blutti in self.bluttis.iter_mut() {
            if let Some(game_state) = Self::update_blutti(blutti) {
                self.game_state = game_state;
                break;
            }
        }
    }

    fn update_blutti(blutti: &mut Blutti) -> Option<GameState> {
        let buttons = read_buttons(blutti.peer);
        let just_pressed = buttons.just_pressed(&blutti.buttons);
        let just_released = buttons.just_released(&blutti.buttons);
        let pad = read_pad(blutti.peer);
        blutti.buttons = buttons;

        if let Some(pad) = pad {
            let x = pad.x;
            let y = pad.y;
            if y > 100 && y > x.abs() {
                blutti.move_up(axis_to_speed(pad.y));
            } else if y < -100 && -y > x.abs() {
                blutti.move_down(axis_to_speed(pad.y));
            } else if x > 100 && x > y.abs() {
                blutti.move_right(axis_to_speed(pad.x));
            } else if x < -100 && -x > y.abs() {
                blutti.move_left(axis_to_speed(pad.x));
            }
        } else {
            blutti.stop();
        }
        if just_pressed.s {
            blutti.start_jump();
        }
        if just_released.s {
            blutti.stop_jump();
        }
        if just_pressed.w {
            blutti.start_dash();
        }
        if just_pressed.e {
            blutti.toggle_debug();
        }
        blutti.update();
        blutti.handle_effects();

        if !blutti.is_alive() || blutti.finished_level {
            Some(GameState::GameOver(blutti.finished_level))
        } else if blutti.died {
            Some(GameState::Died)
        } else {
            None
        }
    }

    pub fn update_animation(&mut self) {
        for blutti in self.bluttis.iter_mut() {
            blutti.animation.update();
        }
    }

    pub fn start_game(&mut self) {
        self.game_state = GameState::Playing;
        self.bluttis = Blutti::build_bluttis(&self.level);
    }

    pub fn reset(&mut self) {
        for blutti in self.bluttis.iter_mut() {
            blutti.reset();
        }
    }

    pub fn restart(&self, won: bool) {
        if won {
            let new_level = self.level.level_number + 1;
            Level::restart(new_level, won);
        } else {
            Level::restart(1, won);
        }
    }
}
