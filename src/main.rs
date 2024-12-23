#![no_std]
#![no_main]

use core::cell::OnceCell;
use firefly_rust::*;

static mut STATE: OnceCell<State> = OnceCell::new();

struct State {
    blutti: Blutti,
}

fn get_state() -> &'static mut State {
    unsafe { STATE.get_mut() }.unwrap()
}

#[no_mangle]
extern "C" fn boot() {
    let state = State {
        blutti: Blutti::default(),
    };
    unsafe { STATE.set(state) }.ok().unwrap();
}

struct Blutti {
    position: Point,
}

impl Default for Blutti {
    fn default() -> Self {
        Self {
            position: Point {
                x: 120,
                y: 160 - Self::SIZE,
            },
        }
    }
}

impl Blutti {
    const SIZE: i32 = 5;
    const SPEED: i32 = 1;

    fn draw(&self) {
        draw_circle(
            self.position,
            Self::SIZE,
            Style {
                fill_color: Color::Red,
                stroke_color: Color::Orange,
                stroke_width: 1,
            },
        );
    }

    fn move_left(&mut self) {
        self.position = Point {
            x: self.position.x - Self::SPEED,
            y: self.position.y,
        }
    }

    fn move_right(&mut self) {
        self.position = Point {
            x: self.position.x + Self::SPEED,
            y: self.position.y,
        }
    }
}

#[no_mangle]
extern "C" fn update() {
    //let buttons = read_buttons(Peer::COMBINED);
    let state = get_state();
    let pad = read_pad(Peer::COMBINED);
    if let Some(pad) = pad {
        let dpad = pad.as_dpad();
        if dpad.left {
            state.blutti.move_left();
        }
        if dpad.right {
            state.blutti.move_right();
        }
    }
}

#[no_mangle]
extern "C" fn render() {
    let state = get_state();
    clear_screen(Color::White);
    state.blutti.draw();
}
