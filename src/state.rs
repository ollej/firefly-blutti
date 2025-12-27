use core::cell::OnceCell;
use firefly_rust::{audio, Buttons, FileBuf};

use crate::{blutti::*, game_state::*, level::*};

pub static mut STATE: OnceCell<State> = OnceCell::new();

pub struct State {
    pub blutti: Blutti,
    pub spritesheet: FileBuf,
    pub title: FileBuf,
    pub font: FileBuf,
    pub fx: audio::Node<audio::Gain>,
    pub theme: audio::Node<audio::Gain>,
    pub level: Level,
    pub game_state: GameState,
    pub buttons: Buttons,
}

pub fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe { STATE.get_mut() }.unwrap()
}
