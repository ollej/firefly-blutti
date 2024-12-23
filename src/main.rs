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
    jump_timer: i32,
    movement: i32,
}

impl Default for Blutti {
    fn default() -> Self {
        Self {
            position: Point {
                x: 120,
                y: 160 - Self::SIZE,
            },
            jump_timer: 0,
            movement: 0,
        }
    }
}

impl Blutti {
    const SIZE: i32 = 5;
    const SPEED: i32 = 1;
    const JUMP_TIME: i32 = 8;
    const JUMP_SPEED: i32 = 1;
    const GRAVITY: i32 = 1;

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
        if self.standing() {
            self.movement -= Self::SPEED;
        }
    }

    fn move_right(&mut self) {
        if self.standing() {
            self.movement += Self::SPEED;
        }
    }

    fn start_jump(&mut self) {
        if self.jump_timer == 0 && self.standing() {
            self.jump_timer = Self::JUMP_TIME;
        }
    }

    fn movement(&mut self) {
        if self.jump_timer > 0 {
            self.position = Point {
                x: self.position.x + self.movement,
                y: self.position.y - Self::JUMP_SPEED,
            };
            self.jump_timer -= 1;
        } else if self.jump_timer == 0 && !self.standing() {
            self.position = Point {
                x: self.position.x + self.movement,
                y: self.position.y + Self::GRAVITY,
            }
        } else if self.movement != 0 && self.standing() {
            self.position = Point {
                x: self.position.x + self.movement,
                y: self.position.y,
            };
            self.movement = 0;
        }
    }

    fn standing(&self) -> bool {
        self.position.y == (Point::MAX.y - Self::SIZE)
    }
}

#[no_mangle]
extern "C" fn update() {
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
    let buttons = read_buttons(Peer::COMBINED);
    if buttons.s {
        state.blutti.start_jump();
    }
    state.blutti.movement();
}

#[no_mangle]
extern "C" fn render() {
    let state = get_state();
    clear_screen(Color::White);
    state.blutti.draw();
}
