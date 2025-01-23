#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::cell::OnceCell;
use firefly_rust::*;
use fixedstr::{str32, str_format};
use serde::Deserialize;

const TILE_WIDTH: i32 = 8;
const TILE_HEIGHT: i32 = 8;
const SPRITES_H: i32 = 16;
const TILES_H: i32 = 30;
const DELTA_TIME: f32 = 1000.0 / 60.0;
const HALF_FONT_WIDTH: i32 = 2;
const FONT_BASE_LINE: i32 = 4;
const LINE_HEIGHT: i32 = 8;
const BADGE_STARS: Badge = Badge(1);
const BADGE_LEVELS: Badge = Badge(2);
const BADGE_DEATHS: Badge = Badge(3);
const BLUTTI_RIGHT_SPRITES: [i32; 2] = [114, 115];
const BLUTTI_LEFT_SPRITES: [i32; 2] = [112, 113];
const BLUTTI_IDLE_RIGHT_SPRITES: [i32; 2] = [118, 119];
const BLUTTI_IDLE_LEFT_SPRITES: [i32; 2] = [116, 117];
const BLUTTI_JUMP_RIGHT_SPRITES: [i32; 4] = [80, 81, 82, 83];
const BLUTTI_JUMP_LEFT_SPRITES: [i32; 4] = [84, 85, 86, 87];
const BLUTTI_CLIMB_RIGHT_SPRITES: [i32; 2] = [120, 121];
const BLUTTI_CLIMB_LEFT_SPRITES: [i32; 2] = [122, 123];
const BLUTTI_DASH_RIGHT_SPRITES: [i32; 4] = [96, 97, 98, 99];
const BLUTTI_DASH_LEFT_SPRITES: [i32; 4] = [100, 101, 102, 103];
const BLUTTI_DEATH_SPRITES: [i32; 4] = [124, 125, 126, 127];
const BLUTTI_EXIT_RIGHT_SPRITES: [i32; 4] = [92, 93, 94, 95];
const BLUTTI_EXIT_LEFT_SPRITES: [i32; 4] = [108, 109, 110, 111];
const COLLECTION_SPRITES: [i32; 4] = [104, 105, 106, 107];

// Level 0 is a debug level
const LEVELS: [&str; 6] = ["level0", "level1", "level2", "level3", "level4", "level5"];

const CREDITS: [&str; 8] = [
    "Credits:",
    "Programming: Olle Wreede",
    "Graphics: Olle Wreede",
    "Level design: Olle Wreede",
    "Music: Zane Little Music",
    "SFX: @Shades, Luke.RUSTLTD, sauer2",
    "",
    "Press (E) to go back to game",
];

const INFO: [&str; 5] = [
    "Controls:",
    "Press (S) to jump",
    "Press (W) to dash",
    "",
    "Press (E) to go back to game",
];

const COLLISION: [TileCollider; 256] = [
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Climbable,
    TileCollider::Star,
    TileCollider::ExtraLife,
    TileCollider::Exit,
    TileCollider::None,
    TileCollider::Deadly,
    TileCollider::Deadly,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Deadly,
    TileCollider::None,
    TileCollider::Full,
    TileCollider::Slippery,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::None,
    TileCollider::None,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::Deadly,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Climbable,
    TileCollider::Full,
    TileCollider::Full,
    TileCollider::Conveyor,
    TileCollider::Conveyor,
    TileCollider::Conveyor,
    TileCollider::Exit,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
    TileCollider::None,
];

trait PointMath {
    fn top_right(&self) -> Point;
    fn bottom_left(&self) -> Point;
    fn bottom_right(&self) -> Point;
}

impl PointMath for Point {
    fn top_right(&self) -> Point {
        Point {
            x: self.x + TILE_WIDTH - 1,
            y: self.y,
        }
    }

    fn bottom_left(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + TILE_HEIGHT - 1,
        }
    }

    fn bottom_right(&self) -> Point {
        Point {
            x: self.x + TILE_WIDTH - 1,
            y: self.y + TILE_HEIGHT - 1,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Deserialize)]
enum TileCollider {
    Full,
    Climbable,
    Star,
    ExtraLife,
    Deadly,
    Slippery,
    Conveyor,
    Exit,
    None,
}

#[derive(Deserialize)]
#[serde(remote = "Point")]
struct PointDef {
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
#[serde(remote = "Color")]
enum ColorDef {
    None,
    Black,
    Purple,
    Red,
    Orange,
    Yellow,
    LightGreen,
    Green,
    DarkGreen,
    DarkBlue,
    Blue,
    LightBlue,
    Cyan,
    White,
    LightGray,
    Gray,
    DarkGray,
}

type Sprite = i32;

struct Collision {
    tile_collider: TileCollider,
    position: Point,
}

#[derive(Clone, Debug, Deserialize)]
struct Level {
    tiles: Vec<Sprite>,
    #[serde(with = "ColorDef")]
    background_color: Color,
    #[serde(with = "ColorDef")]
    font_color: Color,
    stars: i32,
    #[serde(with = "PointDef")]
    start_position: Point,
    particle_chance: i32,
    particle_sprite: i32,
    monsters: Vec<Monster>,
    #[serde(skip)]
    particles: Vec<Particle>,
    #[serde(skip)]
    original_monsters: Vec<Monster>,
}

impl Level {
    const MIN: Point = Point::MIN;
    const MAX: Point = Point {
        x: Point::MAX.x - TILE_WIDTH,
        y: Point::MAX.y - TILE_WIDTH,
    };

    fn load_level(level: i32) -> Self {
        let level_name = LEVELS[level as usize];
        let level_data = load_file_buf(level_name).expect("Couldn't load level data");
        let mut level =
            serde_json::from_slice::<Level>(level_data.data()).expect("Couldn't parse level data");
        level.original_monsters = level.monsters.clone();
        level
    }

    fn update(&mut self) {
        for monster in self.monsters.iter_mut() {
            monster.update();
        }
        if random_value(100) < self.particle_chance {
            self.particles.push(Particle::random(self.particle_sprite));
        }
        for particle in self.particles.iter_mut() {
            particle.update();
        }
        self.particles
            .retain(|particle| !particle.should_be_removed());
    }

    fn draw(&mut self) {
        clear_screen(self.background_color);
        for (&tile, i) in self.tiles.iter().zip(0..) {
            let point = Point {
                x: ((i % TILES_H) * TILE_WIDTH),
                y: ((i / TILES_H) * TILE_HEIGHT),
            };
            if tile > 0 {
                draw_tile(tile - 1, point);
            }
        }
    }

    fn draw_children(&self) {
        for monster in self.monsters.iter() {
            monster.draw();
        }
        for particle in self.particles.iter() {
            particle.draw();
        }
    }

    fn reset(&mut self) {
        self.monsters = self.original_monsters.clone();
    }

    fn sprite_at_position(&self, point: Point) -> Sprite {
        let tile_pos = get_tile_index(point);
        let maybe_sprite = self.tiles.get(tile_pos as usize);
        // The default sprite should be a sprite with TileCollider::Full
        // in the COLLISION list. Otherwise, if the Blutti is standing
        // on the bottom edge of the screen, it will be considered falling.
        maybe_sprite.unwrap_or(&4) - 1
    }

    fn collision_at_position(&self, position: Point) -> Option<Collision> {
        //log_debug(str_format!(str256, "x: {}", test_point.x).as_str());
        //log_debug(str_format!(str256, "y: {}", test_point.y).as_str());
        //log_debug(str_format!(str256, "tile_pos: {}", tile_pos).as_str());
        //log_debug(str_format!(str256, "tile: {}", tile).as_str());
        let sprite = self.sprite_at_position(position);
        if sprite >= 0 {
            Some(Collision {
                tile_collider: COLLISION[sprite as usize],
                position,
            })
        } else {
            None
        }
    }

    fn all_collisions_at_rect(&self, position: Point) -> [Option<Collision>; 4] {
        [
            self.collision_at_position(position),
            self.collision_at_position(position.top_right()),
            self.collision_at_position(position.bottom_left()),
            self.collision_at_position(position.bottom_right()),
        ]
    }

    fn remove_tile(&mut self, position: Point) {
        let tile_pos = get_tile_index(position);
        self.tiles[tile_pos as usize] = 0;
    }
}

static mut STATE: OnceCell<State> = OnceCell::new();

struct State {
    blutti: Blutti,
    spritesheet: FileBuf,
    title: FileBuf,
    font: FileBuf,
    fx: audio::Node<audio::Gain>,
    theme: audio::Node<audio::Gain>,
    level: Level,
    game_state: GameState,
    buttons: Buttons,
}

fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe { STATE.get_mut() }.unwrap()
}

enum GameState {
    Playing,
    Title,
    Credits,
    Info,
    Died,
    GameOver(bool),
}

trait Drawable {
    fn draw(&self);
    fn draw_debug(&self);
}

trait Updateable {
    fn update(&mut self);

    fn position(&self) -> Point;

    fn collision(&self, position: Point) -> TileCollider {
        let state = get_state();
        state
            .level
            .collision_at_position(position)
            .map_or(TileCollider::None, |c| c.tile_collider)
    }

    fn is_tile_empty(&self, position: Point) -> bool {
        match self.collision(position) {
            TileCollider::None
            | TileCollider::Star
            | TileCollider::ExtraLife
            | TileCollider::Deadly
            | TileCollider::Exit => true,
            TileCollider::Climbable
            | TileCollider::Slippery
            | TileCollider::Conveyor
            | TileCollider::Full => false,
        }
    }

    fn is_tile_free(&self, position: Point) -> bool {
        let occupied = matches!(
            self.collision(position),
            TileCollider::Full | TileCollider::Slippery | TileCollider::Conveyor
        );
        !occupied
    }

    fn position_below_left_foot(&self) -> Point {
        Point {
            x: self.position().x,
            y: self.position().y + TILE_HEIGHT,
        }
    }

    fn position_below_right_foot(&self) -> Point {
        Point {
            x: self.position().x + TILE_WIDTH - 1,
            y: self.position().y + TILE_HEIGHT,
        }
    }

    fn position_top_right(&self) -> Point {
        Point {
            x: self.position().x + TILE_WIDTH - 1,
            y: self.position().y,
        }
    }

    fn position_bottom_left(&self) -> Point {
        Point {
            x: self.position().x,
            y: self.position().y + TILE_HEIGHT - 1,
        }
    }

    fn position_bottom_right(&self) -> Point {
        Point {
            x: self.position().x + TILE_WIDTH - 1,
            y: self.position().y + TILE_HEIGHT - 1,
        }
    }

    fn is_standing(&self) -> bool {
        !(self.is_tile_empty(self.position_below_left_foot())
            && self.is_tile_empty(self.position_below_right_foot()))
    }

    fn is_standing_on(&self, collision: TileCollider) -> bool {
        self.collision(self.position_below_left_foot()) == collision
            || self.collision(self.position_below_right_foot()) == collision
    }
}

#[derive(PartialEq, Debug)]
enum DirectionX {
    Left,
    Right,
}

#[derive(PartialEq)]
enum DirectionY {
    Up,
    Down,
}

#[derive(PartialEq)]
enum PlayerState {
    Idle,
    Jumping,
    Dashing,
    Running,
    Climbing,
    StopRunning,
    StopClimbing,
}

struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

struct Blutti {
    position: Point,
    start_position: Point,
    jump_timer: i32,
    dash_timer: i32,
    fall_timer: i32,
    direction_x: DirectionX,
    direction_y: DirectionY,
    state: PlayerState,
    velocity: Vec2,
    remainder: Vec2,
    points: i32,
    stars: i32,
    lives: i32,
    iddqd: bool,
    died: bool,
    finished_level: bool,
    current_level: i32,
    current_tile: i32,
    animation: Animation,
    debug: bool,
}

impl Default for Blutti {
    fn default() -> Self {
        Self {
            position: Self::START_POSITION,
            start_position: Self::START_POSITION,
            jump_timer: 0,
            dash_timer: 0,
            fall_timer: 0,
            direction_x: DirectionX::Right,
            direction_y: DirectionY::Up,
            state: PlayerState::Idle,
            velocity: Vec2::zero(),
            remainder: Vec2::zero(),
            points: 0,
            stars: 0,
            lives: 3,
            iddqd: false,
            died: false,
            finished_level: false,
            current_level: 1,
            current_tile: 0,
            animation: Animation::animation_idle_right(),
            debug: false,
        }
    }
}

impl Drawable for Blutti {
    fn draw(&self) {
        if !self.animation.finished {
            let tile = self.animation.current_sprite();
            draw_tile(tile, self.position());
        }

        // Player debug
        if self.debug {
            self.draw_debug();
        }
    }

    fn draw_debug(&self) {
        let mut textpos = self.position().clone();
        //textpos.y -= 4;
        //display_text(str_format!(str32, "{:?}", self.state).as_str(), textpos);
        textpos.y -= 4;
        display_text(
            str_format!(str32, "Y {}", self.position().y).as_str(),
            textpos,
        );
        textpos.y -= 8;
        display_text(
            str_format!(str32, "X {}", self.position().x).as_str(),
            textpos,
        );
        /*
        textpos.y -= 8;
        display_text(
            str_format!(str32, "VY {}", self.movement_y).as_str(),
            textpos,
        );
        textpos.y -= 8;
        display_text(
            str_format!(str32, "VX {}", self.movement_x).as_str(),
            textpos,
        );
        */
    }
}

impl Updateable for Blutti {
    fn position(&self) -> Point {
        self.position
    }

    fn update(&mut self) {
        self.animation.update();

        let (acceleration, target_velocity) = match self.state {
            PlayerState::Running => (0.5, Self::MAX_VELOCITY),
            PlayerState::Jumping => (0.4, Self::MAX_VELOCITY),
            PlayerState::Climbing => (0.0, 0.0),
            PlayerState::Dashing => (1.2, Self::MAX_VELOCITY),
            PlayerState::Idle => (0.0, 0.0),
            PlayerState::StopRunning => (0.3, 0.0),
            PlayerState::StopClimbing => (0.0, 0.0),
        };

        if self.state == PlayerState::StopRunning {
            if self.direction_x == DirectionX::Left {
                self.velocity.x = (self.velocity.x + acceleration).min(target_velocity);
            } else {
                self.velocity.x = (self.velocity.x - acceleration).max(target_velocity);
            }
            if self.velocity.x == 0.0 {
                self.start_idling();
            }
        } else {
            if self.direction_x == DirectionX::Left {
                self.velocity.x = (self.velocity.x - acceleration).max(-target_velocity);
            } else {
                self.velocity.x = (self.velocity.x + acceleration).min(target_velocity);
            }
        }

        // pos += vel * dt + 1/2*dt*dt
        // Vel += acc*dt
        let (acceleration, target_velocity) = match self.state {
            PlayerState::Jumping => (1.0, 2.0),
            PlayerState::Climbing => {
                if self.is_on_ladder() {
                    (0.4, 1.0)
                } else {
                    self.stop_movement();
                    (0.0, 0.0)
                }
            }
            PlayerState::StopClimbing => (0.2, 0.0),
            PlayerState::Dashing => (0.0, 0.0),
            PlayerState::Running | PlayerState::Idle | PlayerState::StopRunning => {
                if self.is_standing() {
                    (0.0, 0.0)
                } else {
                    (0.8, 2.0) // Gravity
                }
            }
        };
        match self.state {
            PlayerState::Climbing => {
                if self.direction_y == DirectionY::Up {
                    self.velocity.y = (self.velocity.y - acceleration).max(-target_velocity);
                } else {
                    self.velocity.y = (self.velocity.y + acceleration).min(target_velocity);
                }
            }
            PlayerState::StopClimbing => {
                if self.direction_y == DirectionY::Up {
                    self.velocity.y = (self.velocity.y + acceleration).min(target_velocity);
                } else {
                    self.velocity.y = (self.velocity.y - acceleration).max(target_velocity);
                }
            }
            PlayerState::Jumping => {
                self.velocity.y = (self.velocity.y + acceleration).min(target_velocity);
                // TODO: double gravity after apex
            }
            PlayerState::Running | PlayerState::Idle | PlayerState::StopRunning => {
                // Gravity
                self.velocity.y = (self.velocity.y + acceleration).min(target_velocity);
            }
            PlayerState::Dashing => (),
        }

        // Move X position
        self.remainder.x += self.velocity.x;
        let amount = math::floor(self.remainder.x + 0.5);
        self.remainder.x -= amount;
        let step = if amount > 0. {
            1
        } else if amount < 0. {
            -1
        } else {
            0
        };
        for _ in 0..amount.abs() as i32 {
            let mut test_pos = if step > 0 {
                self.position_top_right()
            } else {
                self.position()
            };
            test_pos.x += step;
            if test_pos.x >= 0 && test_pos.x < WIDTH && self.is_tile_free(test_pos) {
                self.position.x += step;
            } else {
                self.stop_movement();
                break;
            }
        }

        // Move y position
        self.remainder.y += self.velocity.y;
        let amount = math::floor(self.remainder.y + 0.5);
        self.remainder.y -= amount;
        let step = if amount > 0. {
            1
        } else if amount < 0. {
            -1
        } else {
            0
        };
        for _ in 0..amount.abs() as i32 {
            let mut test_pos = if step > 0 {
                self.position_bottom_left()
            } else {
                self.position()
            };
            test_pos.y += step;
            if test_pos.y >= 0 && test_pos.y < HEIGHT && self.is_tile_free(test_pos) {
                self.position.y += step;
            } else {
                self.stop_movement();
                break;
            }
        }

        /*
            let mut new_x = self.position.x;
            let mut new_y = self.position.y;

            new_x += self.movement_x;
            if self.is_on_ladder() {
                new_y += self.movement_y;
            }
            if self.is_standing_on(TileCollider::Conveyor) {
                new_x += Self::CONVEYOR_SPEED;
            }

            if self.dash_timer > 0 && self.movement_x != 0 {
                new_x = self.position.x + Self::DASH_VELOCITY * self.movement_x;
            }
            if self.jump_timer > 0 {
                new_y -= Self::JUMP_VELOCITY;
                self.jump_timer -= 1;
            }
            if self.jump_timer == 0 && self.dash_timer <= 0 {
                if self.is_standing() {
                    if self.fall_timer > Self::MAX_FALL_HEIGHT {
                        self.die();
                        return;
                    } else if self.fall_timer > 0 {
                        self.fall_timer = 0;
                    }
                } else {
                    new_y += Self::GRAVITY;
                    self.fall_timer += 1;
                }
            }

            if self.dash_timer == 1 {
                self.dash_timer = -Self::DASH_WAIT_TIME;
            } else if self.dash_timer > 1 {
                self.dash_timer -= 1;
            } else if self.dash_timer < 0 {
                self.dash_timer += 1;
            }
            if self.movement_x != 0 && !self.is_standing_on(TileCollider::Slippery) {
                self.start_idling();
            }
            if self.is_standing() {
                self.movement_y = 0;
            }

            let new_position = Point {
                x: new_x.clamp(Level::MIN.x, Level::MAX.x),
                y: new_y.clamp(Level::MIN.y, Level::MAX.y),
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
                    //log_debug(str_format!(str32, "new y: {}", new_position.y).as_str());
                }
            }
        */
    }
}

impl Blutti {
    const RUNNING_ACCELERATION: f32 = 0.6;
    const MAX_VELOCITY: f32 = 2.0;
    const JUMP_VELOCITY: i32 = 5;
    const DASH_VELOCITY: i32 = 8;
    const DASH_WAIT_TIME: i32 = 32;
    const CONVEYOR_SPEED: i32 = 2;
    const GRAVITY: i32 = 2;
    const MAX_FALL_HEIGHT: i32 = 30;
    const START_POSITION: Point = Point {
        x: WIDTH / 2 - TILE_WIDTH,
        y: HEIGHT - TILE_WIDTH - TILE_HEIGHT,
    };

    fn with_start_position(start_position: Point) -> Self {
        Blutti {
            position: start_position,
            start_position,
            ..Blutti::default()
        }
    }

    fn at_new_level(&self, start_position: Point, current_level: i32) -> Self {
        Blutti {
            position: start_position,
            start_position,
            points: self.points,
            lives: self.lives,
            current_level,
            ..Blutti::default()
        }
    }

    fn toggle_debug(&mut self) {
        self.debug = !self.debug
    }

    fn move_left(&mut self, speed: i32) {
        // Change direction
        if self.direction_x != DirectionX::Left {
            self.direction_x = DirectionX::Left;
            // TODO: Add particle for direction change
            self.add_running_animation();
        }
        self.start_running();
    }

    fn move_right(&mut self, speed: i32) {
        // Change direction
        if self.direction_x != DirectionX::Right {
            self.direction_x = DirectionX::Right;
            // TODO: Add particle for direction change
            self.add_running_animation();
        }
        self.start_running();
    }

    fn move_up(&mut self, speed: i32) {
        // Change direction
        if self.direction_y != DirectionY::Up {
            self.direction_y = DirectionY::Up;
            self.add_climbing_animation();
        }
        self.start_climbing();
    }

    fn move_down(&mut self, speed: i32) {
        // Change direction
        if self.direction_y != DirectionY::Down {
            self.direction_y = DirectionY::Down;
            self.add_climbing_animation();
        }
        self.start_climbing();
    }

    fn stop(&mut self) {
        match self.state {
            PlayerState::Running => self.state = PlayerState::StopRunning,
            PlayerState::Climbing => self.state = PlayerState::StopClimbing,
            _ => (),
        }
    }

    fn start_running(&mut self) {
        if self.state != PlayerState::Running && self.is_standing() {
            self.add_running_animation();
            self.state = PlayerState::Running;
        }
    }

    fn start_climbing(&mut self) {
        if self.is_on_ladder() && self.state != PlayerState::Climbing {
            self.state = PlayerState::Climbing;
            self.add_climbing_animation();
            //log_debug("start climbing");
        }
    }

    fn start_jump(&mut self) {
        if self.is_standing() {
            play_sound("sound_jump");
            self.state = PlayerState::Jumping;
            self.add_jump_animation();
        }
    }

    fn start_dash(&mut self) {
        if self.state == PlayerState::Jumping {
            self.state = PlayerState::Dashing;
            self.add_dash_animation();
            play_sound("sound_dash");
        }
    }

    fn start_idling(&mut self) {
        self.state = PlayerState::Idle;
        self.add_idle_animation();
    }

    fn handle_effects(&mut self) {
        let state = get_state();
        if let Some(collision) = state
            .level
            .all_collisions_at_rect(self.position)
            .into_iter()
            .flatten()
            .last()
        {
            match collision.tile_collider {
                TileCollider::Star => self.collect_star(collision),
                TileCollider::ExtraLife => self.collect_extra_life(collision),
                TileCollider::Exit => self.exit(),
                TileCollider::Deadly => self.die(),
                _ => (),
            }
        }
        state.blutti.current_tile = get_tile_index(self.position);
    }

    fn collect_star(&mut self, collision: Collision) {
        self.handle_collection(collision);
        self.add_points(1);
        self.stars += 1;
        add_progress(get_me(), BADGE_STARS, 1);
        play_sound("sound_coin");
    }

    fn collect_extra_life(&mut self, collision: Collision) {
        self.handle_collection(collision);
        self.add_lives(1);
        play_sound("sound_powerup");
    }

    fn handle_collection(&mut self, collision: Collision) {
        let state = get_state();
        state.level.remove_tile(collision.position);
        let particle_position = get_origin_point_of_position(collision.position);
        self.add_collection_particle(particle_position);
    }

    fn exit(&mut self) {
        let state = get_state();
        if self.stars >= state.level.stars {
            self.finish_level();
        } else {
            let tile_pos = get_tile_index(self.position);
            if tile_pos != state.blutti.current_tile {
                play_sound("sound_wrong");
                state.blutti.current_tile = tile_pos;
            }
        }
    }

    fn add_lives(&mut self, lives: i32) -> i32 {
        self.lives += lives;
        self.lives
    }

    fn add_points(&mut self, points: i32) -> i32 {
        self.points += points;
        self.points
    }

    fn die(&mut self) {
        if self.iddqd {
            return;
        }
        let state = get_state();
        state.level.reset();
        self.stop_movement();
        self.add_death_animation();
        self.add_lives(-1);
        add_progress(get_me(), BADGE_DEATHS, 1);
        self.died = true;
        play_sound("sound_death");
    }

    fn finish_level(&mut self) {
        play_sound("sound_exit");
        self.finished_level = true;
        self.add_exit_animation();
        add_progress(get_me(), BADGE_LEVELS, 1);
    }

    fn stop_movement(&mut self) {
        self.jump_timer = 0;
        self.dash_timer = 0;
        self.fall_timer = 0;
        self.remainder = Vec2::zero();
        self.velocity = Vec2::zero();
        self.start_idling();
    }

    fn reset(&mut self) {
        self.died = false;
        self.direction_x = DirectionX::Right;
        self.position = self.start_position;
        self.stop_movement();
        self.current_tile = 0;
    }

    fn is_on_ladder(&self) -> bool {
        match self.direction_y {
            DirectionY::Up => self.is_on_ladder_up(),
            DirectionY::Down => self.is_on_ladder_down(),
        }
    }

    fn is_on_ladder_up(&self) -> bool {
        self.collision(self.position_bottom_left()) == TileCollider::Climbable
            || self.collision(self.position_bottom_right()) == TileCollider::Climbable
    }

    fn is_on_ladder_down(&self) -> bool {
        self.collision(self.position_below_left_foot()) == TileCollider::Climbable
            || self.collision(self.position_below_right_foot()) == TileCollider::Climbable
    }

    fn is_alive(&self) -> bool {
        self.lives > 0
    }

    fn add_idle_animation(&mut self) {
        self.animation = match self.direction_x {
            DirectionX::Left => Animation::animation_idle_left(),
            DirectionX::Right => Animation::animation_idle_right(),
        }
    }

    fn add_running_animation(&mut self) {
        self.animation = match self.direction_x {
            DirectionX::Left => Animation::animation_running_left(),
            DirectionX::Right => Animation::animation_running_right(),
        }
    }

    fn add_death_animation(&mut self) {
        self.animation = Animation::animation_death()
    }

    fn add_exit_animation(&mut self) {
        self.animation = match self.direction_x {
            DirectionX::Left => Animation::animation_exit_left(),
            DirectionX::Right => Animation::animation_exit_right(),
        }
    }

    fn add_climbing_animation(&mut self) {
        if self.velocity.y == 0.0 {
            self.animation = match self.direction_x {
                DirectionX::Right => Animation::animation_climb_right(),
                DirectionX::Left => Animation::animation_climb_left(),
            }
        }
    }

    fn add_jump_animation(&self) {
        match self.direction_x {
            DirectionX::Left => self.add_jump_particle(BLUTTI_JUMP_LEFT_SPRITES),
            DirectionX::Right => self.add_jump_particle(BLUTTI_JUMP_RIGHT_SPRITES),
        }
    }

    fn add_dash_animation(&self) {
        match self.direction_x {
            DirectionX::Left => self.add_dash_particle(BLUTTI_DASH_LEFT_SPRITES, TILE_WIDTH),
            DirectionX::Right => self.add_dash_particle(BLUTTI_DASH_RIGHT_SPRITES, -TILE_WIDTH),
        }
    }

    fn add_dash_particle(&self, sprites: [i32; 4], offset_x: i32) {
        let anim = Animation::once(sprites.into(), 5);
        let position = Point {
            x: self.position.x + offset_x,
            y: self.position.y,
        };
        let particle = Particle::following(position, anim, offset_x);
        let state = get_state();
        state.level.particles.push(particle);
    }

    fn add_jump_particle(&self, sprites: [i32; 4]) {
        self.add_particle(self.position_bottom_left(), sprites.into());
    }

    fn add_collection_particle(&self, position: Point) {
        self.add_particle(position, COLLECTION_SPRITES.into());
    }

    fn add_particle(&self, position: Point, sprites: Vec<i32>) {
        let anim = Animation::once(sprites, 5);
        let particle = Particle::stationary(position, anim);
        let state = get_state();
        state.level.particles.push(particle);
    }
}

#[derive(PartialEq, Clone, Debug)]
enum ParticleMovement {
    Stationary,
    Falling,
    Following(i32),
}

#[derive(Clone, Debug)]
struct Particle {
    position: Point,
    animation: Animation,
    movement: ParticleMovement,
}

impl Particle {
    const SPEED: i32 = 1;

    fn new(position: Point, animation: Animation, movement: ParticleMovement) -> Self {
        Particle {
            position,
            animation,
            movement,
        }
    }

    fn stationary(position: Point, animation: Animation) -> Self {
        Self::new(position, animation, ParticleMovement::Stationary)
    }

    fn following(position: Point, animation: Animation, offset_x: i32) -> Self {
        Self::new(position, animation, ParticleMovement::Following(offset_x))
    }

    fn random(sprite: i32) -> Self {
        Particle {
            position: Point {
                x: random_value(WIDTH + 64),
                y: -3,
            },
            animation: Animation::looping([sprite, sprite + 1], 15),
            movement: ParticleMovement::Falling,
        }
    }

    fn update(&mut self) {
        self.animation.update();
        self.update_movement();
    }

    fn update_movement(&mut self) {
        match self.movement {
            ParticleMovement::Falling => {
                let new_x = match random_value(100) {
                    90.. => self.position.x + Self::SPEED,
                    60..90 => self.position.x - Self::SPEED,
                    _ => self.position.x,
                };
                let new_y = match random_value(100) {
                    90.. => self.position.y - Self::SPEED,
                    40..90 => self.position.y + Self::SPEED,
                    _ => self.position.y,
                };
                self.position = Point { x: new_x, y: new_y };
            }
            ParticleMovement::Following(offset_x) => {
                let state = get_state();
                let position = state.blutti.position;
                self.position = Point {
                    x: position.x + offset_x,
                    y: position.y,
                }
            }
            ParticleMovement::Stationary => (),
        }
    }

    fn should_be_removed(&self) -> bool {
        self.animation.finished || self.position.y > HEIGHT
    }
}

impl Drawable for Particle {
    fn draw(&self) {
        draw_tile(self.animation.current_sprite(), self.position);
    }

    fn draw_debug(&self) {}
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(from = "MonsterSerde")]
struct Monster {
    #[serde(with = "PointDef")]
    position: Point,
    sprite: i32,
    movement: i32,
    #[serde(skip)]
    animation: Animation,
}

#[derive(Deserialize)]
struct MonsterSerde {
    #[serde(with = "PointDef")]
    position: Point,
    sprite: i32,
    movement: i32,
}

impl From<MonsterSerde> for Monster {
    fn from(value: MonsterSerde) -> Monster {
        Monster {
            position: value.position,
            movement: value.movement,
            sprite: value.sprite,
            animation: Monster::animation_from(value.movement, value.sprite),
        }
    }
}

impl Monster {
    fn animation_from(movement: i32, sprite: i32) -> Animation {
        if movement >= 0 {
            Animation::looping([sprite + 2, sprite + 3], 10)
        } else {
            Animation::looping([sprite, sprite + 1], 10)
        }
    }

    fn change_direction(&mut self) {
        self.movement *= -1;
        self.animation = Self::animation_from(self.movement, self.sprite);
    }
}

impl Drawable for Monster {
    fn draw(&self) {
        draw_tile(self.animation.current_sprite(), self.position());
    }

    fn draw_debug(&self) {}
}

impl Updateable for Monster {
    fn update(&mut self) {
        self.animation.update();

        let new_x = self.position.x + self.movement;
        let test_x = if new_x > self.position.x {
            new_x + TILE_WIDTH - 1
        } else {
            new_x
        };
        let level_boundaries = Level::MIN.x..Level::MAX.x;
        if level_boundaries.contains(&new_x)
            && self.is_tile_free(Point {
                x: test_x,
                y: self.position.y,
            })
            && !self.is_tile_free(Point {
                x: test_x,
                y: self.position.y + TILE_HEIGHT,
            })
        {
            self.position.x = new_x;
        } else {
            self.change_direction();
        }

        let state = get_state();
        let pos = state.blutti.position;

        // Check for death
        if self.position.x <= pos.x + TILE_WIDTH
            && (self.position.x + TILE_WIDTH) >= pos.x
            && self.position.y <= (pos.y + TILE_HEIGHT)
            && (self.position.y + TILE_WIDTH) >= pos.y
        {
            state.blutti.die();
        }
    }

    fn position(&self) -> Point {
        self.position
    }
}

#[derive(Clone, Default, Debug)]
struct Animation {
    sprites: Vec<i32>,
    current_frame: i32,
    time_per_frame: i32,
    frame_timer: i32,
    looping: bool,
    finished: bool,
}

impl Animation {
    fn new(sprites: Vec<i32>, time_per_frame: i32, looping: bool) -> Self {
        Self {
            sprites,
            current_frame: 0,
            time_per_frame,
            frame_timer: 0,
            looping,
            finished: false,
        }
    }

    fn animation_idle_left() -> Animation {
        Animation::looping(BLUTTI_IDLE_LEFT_SPRITES, 10)
    }

    fn animation_idle_right() -> Animation {
        Animation::looping(BLUTTI_IDLE_RIGHT_SPRITES, 10)
    }

    fn animation_running_left() -> Animation {
        Animation::looping(BLUTTI_LEFT_SPRITES, 10)
    }

    fn animation_running_right() -> Animation {
        Animation::looping(BLUTTI_RIGHT_SPRITES, 10)
    }

    fn animation_climb_left() -> Animation {
        Animation::looping(BLUTTI_CLIMB_LEFT_SPRITES, 10)
    }

    fn animation_climb_right() -> Animation {
        Animation::looping(BLUTTI_CLIMB_RIGHT_SPRITES, 10)
    }

    fn animation_death() -> Animation {
        Animation::once(BLUTTI_DEATH_SPRITES.into(), 5)
    }

    fn animation_exit_left() -> Animation {
        Animation::once(BLUTTI_EXIT_LEFT_SPRITES.into(), 5)
    }

    fn animation_exit_right() -> Animation {
        Animation::once(BLUTTI_EXIT_RIGHT_SPRITES.into(), 5)
    }

    fn once(sprites: Vec<i32>, time_per_frame: i32) -> Self {
        Self::new(sprites, time_per_frame, false)
    }

    fn looping(sprites: [i32; 2], time_per_frame: i32) -> Self {
        Self::new(sprites.into(), time_per_frame, true)
    }

    fn current_sprite(&self) -> i32 {
        self.sprites[self.current_frame as usize]
    }

    fn update(&mut self) {
        self.frame_timer += 1;
        if self.frame_timer > self.time_per_frame {
            self.next_frame();
        }
    }

    fn next_frame(&mut self) {
        self.frame_timer = 0;
        self.current_frame += 1;
        if self.current_frame >= self.sprites.len() as i32 {
            self.current_frame = 0;
            if !self.looping {
                self.finished = true;
            }
        }
    }
}

fn play_sound(sound: &str) {
    let state = get_state();
    state.fx.clear();
    state.fx.add_file(sound);
}

fn play_music(sound: &str) {
    let state = get_state();
    state.theme.clear();
    state.theme.add_file(sound);
}

// # Scale {value} between {min} and {max} into 0..1
///
/// ```
/// assert_eq!(scale(0, 10, 1), 0.1);
/// assert_eq!(scale(0, 100, 45), 0.45);
/// ```
fn scale(min: u32, max: u32, value: u32) -> f32 {
    (value as f32 - min as f32) / (max as f32 - min as f32)
}

// # Random value between 0 and {max}
fn random_value(max: i32) -> i32 {
    let rnd = get_random();
    math::floor(scale(u32::MIN, u32::MAX, rnd) * max as f32) as i32
}

fn get_origin_point_of_position(position: Point) -> Point {
    Point {
        x: position.x / TILE_WIDTH * TILE_WIDTH,
        y: position.y / TILE_HEIGHT * TILE_HEIGHT,
    }
}

fn get_tile_index(point: Point) -> i32 {
    let tile_x = point.x / TILE_WIDTH;
    let tile_y = point.y / TILE_WIDTH;
    //log_debug(str_format!(str256, "tile_x: {} tile_y: {}", tile_x, tile_y).as_str());
    tile_y * TILES_H + tile_x
}

fn draw_tile(sprite: i32, point: Point) {
    let state = get_state();
    let tile_sprite = state.spritesheet.as_image().sub(
        Point {
            x: ((sprite % SPRITES_H) * TILE_WIDTH),
            y: ((sprite / SPRITES_H) * TILE_HEIGHT),
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
    let color = state.level.font_color;
    display_text_color(text, position, color);
}

fn display_text_color(text: &str, position: Point, color: Color) {
    let state = get_state();
    let font = state.font.as_font();
    draw_text(text, &font, position, color);
}

fn display_centered_message(color: Option<Color>, lines: &[&str]) {
    let state = get_state();
    let color = color.unwrap_or(state.level.font_color);
    let y_pos: i32 = HEIGHT / 2 + FONT_BASE_LINE - lines.len() as i32 * LINE_HEIGHT / 2;
    for (i, line) in lines.iter().enumerate() {
        display_text_color(
            line,
            Point {
                x: WIDTH / 2 - (line.len() as i32 * HALF_FONT_WIDTH),
                y: y_pos + i as i32 * LINE_HEIGHT,
            },
            color,
        );
    }
}

fn display_left_message(lines: &[&str]) {
    let y_pos: i32 = FONT_BASE_LINE + 4;
    for (i, line) in lines.iter().enumerate() {
        display_text_color(
            line,
            Point {
                x: 4,
                y: y_pos + i as i32 * LINE_HEIGHT,
            },
            Color::Black,
        );
    }
}

fn restart(mut level: i32, won: bool) -> i32 {
    let state = get_state();
    if level >= LEVELS.len() as i32 {
        // Restart at level 1, as level 0 is a debug level
        level = 1;
    }
    state.level = Level::load_level(level);
    if won {
        state.blutti = state.blutti.at_new_level(state.level.start_position, level);
        state.game_state = GameState::Playing;
    } else {
        state.blutti = Blutti::with_start_position(state.level.start_position);
        state.level.reset();
        state.game_state = GameState::Title;
    }
    level
}

fn render_title() {
    let state = get_state();
    draw_image(&state.title.as_image(), Point { x: 0, y: 0 });
    display_centered_message(Some(Color::White), &["Press (E) to start!"]);
}

fn render_died() {
    let state = get_state();
    state.level.draw();
    state.blutti.draw();
    render_ui();
    display_centered_message(None, &["You died!", "Press (E) to restart level"]);
}

fn render_gameover(won: bool) {
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

fn render_ui() {
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

fn render_credits() {
    clear_screen(Color::White);
    display_left_message(&CREDITS);
}

fn render_info() {
    clear_screen(Color::White);
    display_left_message(&INFO);
}

fn render_playing() {
    let state = get_state();

    state.level.draw();
    state.blutti.draw();
    state.level.draw_children();
    render_ui();
}

#[no_mangle]
extern "C" fn cheat(cmd: i32, val: i32) -> i32 {
    let state = get_state();
    match cmd {
        1 => restart(val, true),
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

#[no_mangle]
extern "C" fn handle_menu(menu_item: u8) {
    let state = get_state();
    match menu_item {
        1 => state.game_state = GameState::Credits,
        2 => {
            restart(1, false);
        }
        3 => state.game_state = GameState::Info,
        _ => (),
    }
}

#[no_mangle]
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

#[no_mangle]
extern "C" fn update() {
    let state = get_state();
    let buttons = read_buttons(Peer::COMBINED);
    let just_pressed = buttons.just_pressed(&state.buttons);
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
                } else {
                    state.blutti.start_idling();
                }
                if pad.x < -100 {
                    state.blutti.move_left(axis_to_speed(pad.x));
                } else if pad.x > 100 {
                    state.blutti.move_right(axis_to_speed(pad.x));
                } else {
                    state.blutti.start_idling();
                }
            } else {
                state.blutti.stop();
            }
            if just_pressed.s {
                state.blutti.start_jump();
            }
            if just_pressed.w {
                state.blutti.start_dash();
            }
            if just_pressed.e {
                state.blutti.toggle_debug();
            }
            state.blutti.update();
            state.level.update();
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
                    restart(state.blutti.current_level + 1, won);
                } else {
                    restart(1, won);
                }
            }
        }
    }
}

#[inline]
fn axis_to_speed(x: i32) -> i32 {
    if x.abs() > 400 {
        2
    } else {
        1
    }
}

#[no_mangle]
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
