use core::cell::OnceCell;
use firefly_rust::*;

use crate::{blutti::*, functions::*, game_state::*, level::*, updateable::*};

pub static mut STATE: OnceCell<State> = OnceCell::new();

pub fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe { STATE.get_mut() }.unwrap()
}

pub struct State {
    pub blutti: Blutti,
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
        let buttons = read_buttons(Peer::COMBINED);
        let just_pressed = buttons.just_pressed(&self.buttons);
        let just_released = buttons.just_released(&self.buttons);
        let pad = read_pad(Peer::COMBINED);

        if let Some(pad) = pad {
            let x = pad.x;
            let y = pad.y;
            if y > 100 && y > x.abs() {
                self.blutti.move_up(axis_to_speed(pad.y));
            } else if y < -100 && -y > x.abs() {
                self.blutti.move_down(axis_to_speed(pad.y));
            } else if x > 100 && x > y.abs() {
                self.blutti.move_right(axis_to_speed(pad.x));
            } else if x < -100 && -x > y.abs() {
                self.blutti.move_left(axis_to_speed(pad.x));
            }
        } else {
            self.blutti.stop();
        }
        if just_pressed.s {
            self.blutti.start_jump();
        }
        if just_released.s {
            self.blutti.stop_jump();
        }
        if just_pressed.w {
            self.blutti.start_dash();
        }
        if just_pressed.e {
            self.blutti.toggle_debug();
        }
        self.level.update();
        self.blutti.update();
        self.blutti.handle_effects();

        if !self.blutti.is_alive() || self.blutti.finished_level {
            self.game_state = GameState::GameOver(self.blutti.finished_level);
        } else if self.blutti.died {
            self.game_state = GameState::Died;
        }
    }
}
