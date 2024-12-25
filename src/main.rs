#![no_std]
#![no_main]

use core::cell::OnceCell;
use firefly_rust::*;
use fixedstr::{str256, str32, str_format};

const TILE_WIDTH: i32 = 8;
const TILE_HEIGHT: i32 = 8;
const SPRITES_H: i32 = 8;
const SPRITES_V: i32 = 8;
const TILES_H: i32 = 30;
const TILES_V: i32 = 20;

#[rustfmt::skip]
const LEVEL: [i32; 600] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 10, 0, 0, 10, 0, 10, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3, 9, 3, 3, 3, 3, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 10, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 4, 3, 3, 3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 11, 0, 0,
    3, 3, 3, 3, 8, 8, 8, 8, 8, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
];

static mut STATE: OnceCell<State> = OnceCell::new();

struct State {
    blutti: Blutti,
    spritesheet: FileBuf,
    tiles: [TileCollider; 64],
    collected: [bool; 600],
    font: FileBuf,
    fx: audio::Node<audio::Gain>,
}

fn get_state() -> &'static mut State {
    unsafe { STATE.get_mut() }.unwrap()
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum TileCollider {
    Full,
    Top,
    Climbable,
    Collectable,
    None,
}

struct Blutti {
    position: Point,
    jump_timer: i32,
    dash_timer: i32,
    movement: i32,
    direction: Direction,
    points: i32,
    lives: i32,
}

impl Default for Blutti {
    fn default() -> Self {
        Self {
            position: Point {
                x: WIDTH / 2 - Self::SIZE,
                y: HEIGHT - Self::SIZE - TILE_HEIGHT,
            },
            jump_timer: 0,
            dash_timer: 0,
            movement: 0,
            points: 0,
            direction: Direction::Left,
            lives: 3,
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
    const GRAVITY: i32 = 2;
    const MIN: Point = Point::MIN;
    const MAX: Point = Point {
        x: Point::MAX.x - Self::SIZE,
        y: Point::MAX.y - Self::SIZE,
    };

    fn draw(&self) {
        let tile = match self.direction {
            Direction::Left | Direction::Up => 1,
            Direction::Right | Direction::Down => 2,
        };
        draw_tile(tile, self.position);
    }

    fn move_left(&mut self) {
        self.direction = Direction::Left;
        self.movement = -Self::SPEED;
    }

    fn move_right(&mut self) {
        self.direction = Direction::Right;
        self.movement = Self::SPEED;
    }

    fn move_up(&mut self) {
        self.direction = Direction::Up;
        self.movement = -Self::SPEED;
    }

    fn move_down(&mut self) {
        self.direction = Direction::Down;
        self.movement = Self::SPEED;
    }

    fn start_jump(&mut self) {
        if self.jump_timer == 0 && self.is_standing() {
            self.play_sound("sound_jump");
            self.jump_timer = Self::JUMP_TIME;
        }
    }

    fn start_dash(&mut self) {
        if self.dash_timer == 0 {
            self.dash_timer = Self::DASH_TIME;
        }
    }

    fn movement(&mut self) {
        let mut new_x = self.position.x;
        let mut new_y = self.position.y;

        match self.direction {
            Direction::Left | Direction::Right => new_x += self.movement,
            Direction::Up | Direction::Down => {
                if self.is_on_ladder() {
                    new_y += self.movement;
                }
            }
        };

        if self.dash_timer > 0
            && (self.direction == Direction::Left || self.direction == Direction::Right)
        {
            new_x = self.position.x + Self::DASH_SPEED * self.movement;
        }
        if self.jump_timer > 0 {
            new_y -= Self::JUMP_SPEED;
            self.jump_timer -= 1;
        }
        if self.jump_timer == 0 && self.dash_timer <= 0 {
            if !self.is_standing() {
                new_y += Self::GRAVITY;
            }
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
                new_position.x + TILE_WIDTH - 1
            } else {
                new_position.x
            };
            if self.is_tile_free(Point {
                x: test_x,
                y: self.position.y,
            }) {
                self.position.x = new_position.x;
            }
        }
        if self.position.y != new_position.y {
            let test_y = if new_position.y > self.position.y {
                new_position.y + TILE_HEIGHT - 1
            } else {
                new_position.y
            };
            if self.is_tile_free(Point {
                x: self.position.x,
                y: test_y,
            }) {
                self.position.y = new_position.y;
            }
        }
    }

    fn handle_effects(&mut self) {
        match self.current_tile() {
            TileCollider::Collectable => self.collect_item(),
            _ => (),
        }
    }

    fn collect_item(&mut self) {
        let state = get_state();
        let tile_pos = get_tile_index(self.position);
        if !state.collected[tile_pos] {
            match LEVEL[tile_pos] {
                10 => {
                    self.points += 1;
                    self.play_sound("sound_coin");
                }
                11 => {
                    self.lives += 1;
                    self.play_sound("sound_powerup");
                }
                _ => (),
            }
            state.collected[tile_pos] = true;
        }
    }

    fn play_sound(&self, sound: &str) {
        let state = get_state();
        state.fx.clear();
        state.fx.add_file(sound);
    }

    fn current_tile(&self) -> TileCollider {
        self.tile_at_point(self.position)
    }

    fn is_tile_empty(&self, point: Point) -> bool {
        let tile = self.tile_at_point(point);
        tile == TileCollider::None || tile == TileCollider::Collectable
    }

    fn is_tile_free(&self, point: Point) -> bool {
        match self.tile_at_point(point) {
            TileCollider::None | TileCollider::Climbable | TileCollider::Collectable => true,
            TileCollider::Full | TileCollider::Top => false,
        }
    }

    fn tile_at_point(&self, test_point: Point) -> TileCollider {
        //log_debug(str_format!(str256, "x: {}", test_point.x).as_str());
        //log_debug(str_format!(str256, "y: {}", test_point.y).as_str());
        let tile_pos = get_tile_index(test_point);
        //log_debug(str_format!(str256, "tile_pos: {}", tile_pos).as_str());
        let tile = LEVEL[tile_pos as usize];
        //log_debug(str_format!(str256, "tile: {}", tile).as_str());
        let state = get_state();
        state.tiles[tile as usize]
    }

    fn position_below_left_foot(&self) -> Point {
        Point {
            x: self.position.x,
            y: self.position.y + TILE_HEIGHT,
        }
    }

    fn position_below_right_foot(&self) -> Point {
        Point {
            x: self.position.x + TILE_WIDTH - 1,
            y: self.position.y + TILE_HEIGHT,
        }
    }

    fn position_left_foot(&self) -> Point {
        Point {
            x: self.position.x,
            y: self.position.y + TILE_HEIGHT - 1,
        }
    }

    fn position_right_foot(&self) -> Point {
        Point {
            x: self.position.x + TILE_WIDTH - 1,
            y: self.position.y + TILE_HEIGHT - 1,
        }
    }

    fn is_standing(&self) -> bool {
        !(self.is_tile_empty(self.position_below_left_foot())
            && self.is_tile_empty(self.position_below_right_foot()))
    }

    fn is_on_ladder(&self) -> bool {
        self.tile_at_point(self.position_below_left_foot()) == TileCollider::Climbable
            || self.tile_at_point(self.position_below_right_foot()) == TileCollider::Climbable
            || self.tile_at_point(self.position_left_foot()) == TileCollider::Climbable
            || self.tile_at_point(self.position_right_foot()) == TileCollider::Climbable
    }
}

fn get_tile_index(point: Point) -> usize {
    let tile_x = point.x / TILE_WIDTH;
    let tile_y = point.y / TILE_WIDTH;
    //log_debug(str_format!(str256, "tile_x: {} tile_y: {}", tile_x, tile_y).as_str());
    (tile_y * TILES_H + tile_x) as usize
}

fn draw_tile(pos: i32, point: Point) {
    let state = get_state();
    let tile_sprite = state.spritesheet.as_image().sub(
        Point {
            x: ((pos % SPRITES_H) * TILE_WIDTH),
            y: ((pos / SPRITES_H) * TILE_HEIGHT),
        },
        Size {
            width: 8,
            height: 8,
        },
    );
    draw_sub_image(&tile_sprite, point);
}

fn display_text(text: &str, position: Point) {
    let state = get_state();
    let font = state.font.as_font();
    draw_text(text, &font, position, Color::Black);
}

fn render_ui() {
    let state = get_state();
    display_text(
        str_format!(str32, "Points: {}", state.blutti.points).as_str(),
        Point { x: 4, y: 10 },
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

fn render_level() {
    let state = get_state();
    for (i, tile) in LEVEL.iter().enumerate() {
        let mut tile = *tile;
        if state.collected[i] {
            tile = 0;
        }
        let point = Point {
            x: ((i as i32 % TILES_H) * TILE_WIDTH),
            y: ((i as i32 / TILES_H) * TILE_HEIGHT),
        };
        draw_tile(tile, point);
    }
}

#[no_mangle]
extern "C" fn boot() {
    let mut tiles = [TileCollider::None; 64];
    tiles[3] = TileCollider::Full;
    tiles[4] = TileCollider::Full;
    tiles[5] = TileCollider::Full;
    tiles[6] = TileCollider::Full;
    tiles[7] = TileCollider::Full;
    tiles[8] = TileCollider::Full;
    tiles[9] = TileCollider::Climbable;
    tiles[10] = TileCollider::Collectable;
    tiles[11] = TileCollider::Collectable;
    let fx = audio::OUT.add_gain(0.5);
    let state = State {
        blutti: Blutti::default(),
        spritesheet: load_file_buf("spritesheet").unwrap(),
        tiles,
        collected: [false; 600],
        font: load_file_buf("font").unwrap(),
        fx,
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
        if dpad.up {
            state.blutti.move_up();
        }
        if dpad.down {
            state.blutti.move_down();
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
    state.blutti.handle_effects();
}

#[no_mangle]
extern "C" fn render() {
    let state = get_state();
    clear_screen(Color::White);
    render_level();
    state.blutti.draw();
    render_ui();
}
