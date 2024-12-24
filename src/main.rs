#![no_std]
#![no_main]

use core::cell::OnceCell;
use firefly_rust::*;
use fixedstr::{str256, str_format};

const TILE_WIDTH: u8 = 8;
const TILE_HEIGHT: u8 = 8;
const SPRITES_H: u8 = 8;
const SPRITES_V: u8 = 8;
const TILES_H: u8 = 30;
const TILES_V: u8 = 20;

#[rustfmt::skip]
const LEVEL: [u8; 600] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 4, 3, 3, 3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 3, 3, 3, 8, 8, 8, 8, 8, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
];

static mut STATE: OnceCell<State> = OnceCell::new();

struct State {
    blutti: Blutti,
    spritesheet: FileBuf,
}

fn get_state() -> &'static mut State {
    unsafe { STATE.get_mut() }.unwrap()
}

enum Direction {
    Left,
    Right,
}

struct Blutti {
    position: Point,
    jump_timer: i32,
    dash_timer: i32,
    movement: i32,
    direction: Direction,
}

impl Default for Blutti {
    fn default() -> Self {
        Self {
            position: Point {
                x: 120 - Self::SIZE,
                y: 160 - Self::SIZE - TILE_HEIGHT as i32,
            },
            jump_timer: 0,
            dash_timer: 0,
            movement: 0,
            direction: Direction::Left,
        }
    }
}

impl Blutti {
    const SIZE: i32 = 8;
    const SPEED: i32 = 2;
    const JUMP_TIME: i32 = 8;
    const JUMP_SPEED: i32 = 2;
    const DASH_TIME: i32 = 8;
    const DASH_WAIT_TIME: i32 = 32;
    const DASH_SPEED: i32 = 1;
    const GRAVITY: i32 = 1;
    const MIN: Point = Point::MIN;
    const MAX: Point = Point {
        x: Point::MAX.x - Self::SIZE,
        y: Point::MAX.y - Self::SIZE,
    };

    fn draw(&self) {
        let state = get_state();
        let blutti = match self.direction {
            Direction::Left => state.spritesheet.as_image().sub(
                Point { x: 8, y: 0 },
                Size {
                    width: 8,
                    height: 8,
                },
            ),
            Direction::Right => state.spritesheet.as_image().sub(
                Point { x: 16, y: 0 },
                Size {
                    width: 8,
                    height: 8,
                },
            ),
        };
        draw_sub_image(&blutti, self.position);
    }

    fn move_left(&mut self) {
        self.direction = Direction::Left;
        self.movement = -Self::SPEED;
    }

    fn move_right(&mut self) {
        self.direction = Direction::Right;
        self.movement = Self::SPEED;
    }

    fn start_jump(&mut self) {
        if self.jump_timer == 0 && self.standing() {
            self.jump_timer = Self::JUMP_TIME;
        }
    }

    fn start_dash(&mut self) {
        if self.dash_timer == 0 {
            self.dash_timer = Self::DASH_TIME;
        }
    }

    fn movement(&mut self) {
        let mut new_x = self.position.x + self.movement;
        let mut new_y = self.position.y;

        if self.dash_timer > 0 {
            new_x = self.position.x + Self::DASH_SPEED * self.movement;
        }
        if self.jump_timer > 0 {
            new_y -= Self::JUMP_SPEED;
            self.jump_timer -= 1;
        }
        if self.jump_timer == 0 && self.dash_timer <= 0 {
            new_y += Self::GRAVITY;
        }

        if self.dash_timer == 1 {
            self.dash_timer = -Self::DASH_WAIT_TIME;
        } else if self.dash_timer > 1 {
            self.dash_timer -= 1;
        } else if self.dash_timer < 0 {
            self.dash_timer += 1;
        }
        if self.movement != 0 {
            self.movement = 0;
        }

        let new_position = Point {
            x: new_x.clamp(Self::MIN.x, Self::MAX.x),
            y: new_y.clamp(Self::MIN.y, Self::MAX.y),
        };

        if self.position.x != new_position.x {
            let test_x = if new_position.x > self.position.x {
                new_position.x + TILE_WIDTH as i32 - 1
            } else {
                new_position.x
            };
            if self.is_tile_empty(Point {
                x: test_x,
                y: self.position.y,
            }) {
                self.position.x = new_position.x;
            }
        }
        if self.position.y != new_position.y {
            let test_y = if new_position.y > self.position.y {
                new_position.y + TILE_HEIGHT as i32 - 1
            } else {
                new_position.y
            };
            if self.is_tile_empty(Point {
                x: self.position.x,
                y: test_y,
            }) {
                self.position.y = new_position.y;
            }
        }
    }

    fn is_tile_empty(&self, test_point: Point) -> bool {
        //log_debug(str_format!(str256, "x: {}", test_point.x).as_str());
        //log_debug(str_format!(str256, "y: {}", test_point.y).as_str());
        let tile_pos = get_tile_index(test_point);
        //log_debug(str_format!(str256, "tile_pos: {}", tile_pos).as_str());
        let level_pos = LEVEL[tile_pos as usize];
        //log_debug(str_format!(str256, "level_pos: {}", level_pos).as_str());
        level_pos == 0
    }

    fn standing(&self) -> bool {
        let position_below = Point {
            x: self.position.x,
            y: self.position.y + TILE_HEIGHT as i32,
        };
        !self.is_tile_empty(position_below)
    }
}

fn get_tile_index(point: Point) -> usize {
    let tile_x = point.x / TILE_WIDTH as i32;
    let tile_y = point.y / TILE_WIDTH as i32;
    //log_debug(str_format!(str256, "tile_x: {} tile_y: {}", tile_x, tile_y).as_str());
    (tile_y * TILES_H as i32 + tile_x) as usize
}

fn render_level() {
    let state = get_state();
    let sheet = state.spritesheet.as_image();
    for (i, tile) in LEVEL.iter().enumerate() {
        let tile_sprite = sheet.sub(
            Point {
                x: ((tile % SPRITES_H) * TILE_WIDTH) as i32,
                y: ((tile / SPRITES_H) * TILE_HEIGHT) as i32,
            }, // FIXME: calc row
            Size {
                width: 8,
                height: 8,
            },
        );
        draw_sub_image(
            &tile_sprite,
            Point {
                x: ((i as u16 % TILES_H as u16) * TILE_WIDTH as u16) as i32,
                y: ((i as u16 / TILES_H as u16) * TILE_HEIGHT as u16) as i32,
            },
        );
    }
}

#[no_mangle]
extern "C" fn boot() {
    let state = State {
        blutti: Blutti::default(),
        spritesheet: load_file_buf("spritesheet").unwrap(),
    };
    unsafe { STATE.set(state) }.ok().unwrap();
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
    if buttons.w {
        state.blutti.start_dash();
    }
    state.blutti.movement();
}

#[no_mangle]
extern "C" fn render() {
    let state = get_state();
    clear_screen(Color::White);
    render_level();
    state.blutti.draw();
}
