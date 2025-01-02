#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::cell::OnceCell;
use firefly_rust::*;
use fixedstr::{str256, str32, str_format};
use serde::Deserialize;
use serde_json;

const TILE_WIDTH: i32 = 8;
const TILE_HEIGHT: i32 = 8;
const SPRITES_H: i32 = 8;
const SPRITES_V: i32 = 8;
const TILES_H: i32 = 30;
const TILES_V: i32 = 20;
const HALF_FONT_WIDTH: i32 = 2;
const FONT_HEIGHT: i32 = 6;
const FONT_BASE_LINE: i32 = 4;
const LINE_HEIGHT: i32 = 8;
const BADGE_STARS: Badge = Badge(1);
const BADGE_LEVELS: Badge = Badge(2);
const BADGE_DEATHS: Badge = Badge(3);

const LEVELS: [&str; 3] = ["level1", "level2", "level3"];

const CREDITS: [&str; 8] = [
    "Credits:",
    "Programming: Olle Wreede",
    "Graphics: Olle Wreede",
    "Level design: Olle Wreede",
    "Music: Zane Little Music",
    "SFX: @Shades, Luke.RUSTLTD, sauer2",
    "",
    "Press <Y> to go back to game",
];

const INFO: [&str; 5] = [
    "Controls:",
    "Press <A> to jump",
    "Press <X> to dash",
    "",
    "Press <Y> to go back to game",
];

const COLLISION: [TileCollider; 64] = [
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
    TileCollider::Collectable,
    TileCollider::Collectable,
    TileCollider::Collectable,
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
    Collectable,
    Deadly,
    Slippery,
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
    sprite: i32,
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
    monsters: Vec<Monster>,
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

    fn reset(&mut self) {
        self.monsters = self.original_monsters.clone();
    }

    fn sprite_at_pos(&self, tile_pos: usize) -> Sprite {
        self.tiles[tile_pos] - 1
    }

    fn sprite_at_position(&self, point: Point) -> Sprite {
        let tile_pos = get_tile_index(point);
        self.sprite_at_pos(tile_pos)
    }

    fn collision_at_position(&self, position: Point) -> Option<Collision> {
        //log_debug(str_format!(str256, "x: {}", test_point.x).as_str());
        //log_debug(str_format!(str256, "y: {}", test_point.y).as_str());
        //log_debug(str_format!(str256, "tile_pos: {}", tile_pos).as_str());
        //log_debug(str_format!(str256, "tile: {}", tile).as_str());
        let sprite = self.sprite_at_position(position);
        if sprite >= 0 {
            Some(Collision {
                sprite,
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

    fn collect_item(&mut self, position: Point) {
        let tile_pos = get_tile_index(position);
        self.tiles[tile_pos] = 0;
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
    unsafe { STATE.get_mut() }.unwrap()
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
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
            TileCollider::None | TileCollider::Collectable | TileCollider::Deadly => true,
            TileCollider::Climbable | TileCollider::Slippery | TileCollider::Full => false,
        }
    }

    fn is_tile_free(&self, position: Point) -> bool {
        match self.collision(position) {
            TileCollider::Full | TileCollider::Slippery => false,
            _ => true,
        }
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

    fn position_left_foot(&self) -> Point {
        Point {
            x: self.position().x,
            y: self.position().y + TILE_HEIGHT - 1,
        }
    }

    fn position_right_foot(&self) -> Point {
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

struct Blutti {
    position: Point,
    start_position: Point,
    jump_timer: i32,
    dash_timer: i32,
    fall_timer: i32,
    movement: i32,
    direction: Direction,
    points: i32,
    stars: i32,
    lives: i32,
    died: bool,
    finished_level: bool,
    current_level: i32,
    current_tile: i32,
}

impl Default for Blutti {
    fn default() -> Self {
        Self {
            position: Self::START_POSITION,
            start_position: Self::START_POSITION,
            jump_timer: 0,
            dash_timer: 0,
            fall_timer: 0,
            movement: 0,
            points: 0,
            stars: 0,
            direction: Direction::Left,
            lives: 3,
            died: false,
            finished_level: false,
            current_level: 0,
            current_tile: 0,
        }
    }
}

impl Drawable for Blutti {
    fn draw(&self) {
        if self.is_alive() {
            let tile = match self.direction {
                Direction::Left | Direction::Up => 1,
                Direction::Right | Direction::Down => 2,
            };
            draw_tile(tile, self.position());
        }
    }
}

impl Updateable for Blutti {
    fn position(&self) -> Point {
        self.position
    }

    fn update(&mut self) {
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
        if self.movement != 0 && !self.is_standing_on(TileCollider::Slippery) {
            self.movement = 0;
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
    }
}

impl Blutti {
    const SPEED: i32 = 2;
    const JUMP_TIME: i32 = 8;
    const JUMP_SPEED: i32 = 2;
    const DASH_TIME: i32 = 8;
    const DASH_WAIT_TIME: i32 = 32;
    const DASH_SPEED: i32 = 3;
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

    fn move_left(&mut self) {
        self.direction = Direction::Left;
        if !(self.movement > 0 && self.is_standing_on(TileCollider::Slippery)) {
            self.movement = -Self::SPEED;
        }
    }

    fn move_right(&mut self) {
        self.direction = Direction::Right;
        if !(self.movement < 0 && self.is_standing_on(TileCollider::Slippery)) {
            self.movement = Self::SPEED;
        }
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
            play_sound("sound_jump");
            self.jump_timer = if self.is_standing_on(TileCollider::Slippery) {
                Self::JUMP_TIME / 2
            } else {
                Self::JUMP_TIME
            }
        }
    }

    fn start_dash(&mut self) {
        if self.dash_timer == 0 {
            play_sound("sound_dash");
            self.dash_timer = Self::DASH_TIME;
        }
    }

    fn handle_effects(&mut self) {
        let state = get_state();
        for collision in state
            .level
            .all_collisions_at_rect(self.position)
            .iter()
            .flatten()
        {
            match collision.tile_collider {
                TileCollider::Collectable => self.collect_item(collision),
                TileCollider::Deadly => self.die(),
                _ => (),
            }
        }
        state.blutti.current_tile = get_tile_index(self.position) as i32;
    }

    fn collect_star(&mut self) {
        self.add_points(1);
        self.stars += 1;
        add_progress(get_me(), BADGE_STARS, 1);
    }

    fn add_lives(&mut self, lives: i32) -> i32 {
        self.lives += lives;
        self.lives
    }

    fn add_points(&mut self, points: i32) -> i32 {
        self.points += points;
        self.points
    }

    fn collect_item(&mut self, collission: &Collision) {
        let state = get_state();
        //log_debug(str_format!(str32, "new y: {}", new_position.y).as_str());
        match collission.sprite {
            10 => {
                self.collect_star();
                state.level.collect_item(collission.position);
                play_sound("sound_coin");
            }
            11 => {
                self.add_lives(1);
                state.level.collect_item(collission.position);
                play_sound("sound_powerup");
            }
            12 => {
                if self.stars >= state.level.stars {
                    self.finish_level();
                    play_sound("sound_exit");
                } else {
                    let tile_pos = get_tile_index(self.position) as i32;
                    if tile_pos != state.blutti.current_tile {
                        play_sound("sound_wrong");
                        state.blutti.current_tile = tile_pos;
                    }
                }
            }
            _ => (),
        }
    }

    fn die(&mut self) {
        let state = get_state();
        state.level.reset();
        self.add_lives(-1);
        add_progress(get_me(), BADGE_DEATHS, 1);
        self.reset();
        self.died = true;
        play_sound("sound_death");
    }

    fn finish_level(&mut self) {
        self.finished_level = true;
        add_progress(get_me(), BADGE_LEVELS, 1);
    }

    fn reset(&mut self) {
        self.position = self.start_position;
        self.jump_timer = 0;
        self.dash_timer = 0;
        self.fall_timer = 0;
        self.movement = 0;
        self.direction = Direction::Left;
        self.current_tile = 0;
    }

    fn is_on_ladder(&self) -> bool {
        self.collision(self.position_below_left_foot()) == TileCollider::Climbable
            || self.collision(self.position_below_right_foot()) == TileCollider::Climbable
            || self.collision(self.position_left_foot()) == TileCollider::Climbable
            || self.collision(self.position_right_foot()) == TileCollider::Climbable
    }

    fn is_alive(&self) -> bool {
        self.lives > 0
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
struct Monster {
    #[serde(with = "PointDef")]
    position: Point,
    sprite: i32,
    movement: i32,
}

impl Drawable for Monster {
    fn draw(&self) {
        draw_tile(self.sprite, self.position());
    }
}

impl Updateable for Monster {
    fn update(&mut self) {
        let new_x = self.position.x + self.movement;
        let test_x = if new_x > self.position.x {
            new_x + TILE_WIDTH - 1
        } else {
            new_x
        };
        if new_x >= Level::MIN.x
            && new_x < Level::MAX.x
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
            self.movement *= -1;
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
        level = 0;
    }
    state.level = Level::load_level(level);
    if won {
        state.blutti = state
            .blutti
            .at_new_level(state.level.start_position, level as i32);
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
    clear_screen(Color::Black);
    draw_image(&state.title.as_image(), Point { x: 0, y: 0 });
    display_centered_message(Some(Color::White), &["Press <Y> to start!"]);
}

fn render_died() {
    render_level();
    render_ui();
    display_centered_message(None, &["You died!", "Press <Y> to restart level"]);
}

fn render_gameover(won: bool) {
    render_level();
    render_ui();
    if won {
        display_centered_message(None, &["You win!", "Press <Y> to start next level!"]);
    } else {
        display_centered_message(None, &["Game Over!", "Press <Y> to start again!"]);
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

fn render_monsters() {
    let state = get_state();
    for monster in state.level.monsters.iter() {
        monster.draw();
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

fn render_level() {
    let state = get_state();

    clear_screen(state.level.background_color);
    for (i, &tile) in state.level.tiles.iter().enumerate() {
        let point = Point {
            x: ((i as i32 % TILES_H) * TILE_WIDTH),
            y: ((i as i32 / TILES_H) * TILE_HEIGHT),
        };
        if tile > 0 {
            draw_tile(tile - 1, point);
        }
    }
}

fn render_playing() {
    let state = get_state();

    render_level();
    state.blutti.draw();
    render_monsters();
    render_ui();
}

fn add_lives(lives: i32) -> i32 {
    let state = get_state();
    state.blutti.add_lives(lives)
}

fn add_points(points: i32) -> i32 {
    let state = get_state();
    state.blutti.add_points(points)
}

#[no_mangle]
extern "C" fn cheat(cmd: i32, val: i32) -> i32 {
    match cmd {
        1 => restart(val, true),
        2 => add_lives(val),
        3 => add_points(val),
        _ => 0,
    }
}

#[no_mangle]
extern "C" fn handle_menu(menu_item: u8) {
    let state = get_state();
    match menu_item {
        1 => state.game_state = GameState::Credits,
        2 => {
            restart(0, false);
        }
        3 => state.game_state = GameState::Info,
        _ => (),
    }
}

#[no_mangle]
extern "C" fn boot() {
    let fx = audio::OUT.add_gain(1.0);
    let theme = audio::OUT.add_gain(0.5);
    let level = Level::load_level(0);
    let state = State {
        blutti: Blutti::with_start_position(level.start_position),
        spritesheet: load_file_buf("spritesheet").unwrap(),
        title: load_file_buf("title").unwrap(),
        font: load_file_buf("font").unwrap(),
        fx,
        theme,
        level,
        game_state: GameState::Title,
        buttons: Buttons::default(),
    };
    unsafe { STATE.set(state) }.ok().unwrap();
    add_menu_item(1, "Credits");
    add_menu_item(2, "Restart");
    add_menu_item(3, "Info");
    play_music("sound_theme");
}

#[no_mangle]
extern "C" fn update() {
    let state = get_state();
    let buttons = read_buttons(Peer::COMBINED);
    let just_pressed = buttons.just_pressed(&state.buttons);
    state.buttons = buttons;

    match state.game_state {
        GameState::Title => {
            if just_pressed.n {
                state.game_state = GameState::Playing;
            }
        }
        GameState::Credits => {
            if just_pressed.n {
                state.game_state = GameState::Title;
            }
        }
        GameState::Info => {
            if just_pressed.n {
                state.game_state = GameState::Title;
            }
        }
        GameState::Died => {
            if just_pressed.n {
                state.blutti.died = false;
                state.game_state = GameState::Playing;
            }
        }
        GameState::Playing => {
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
            if just_pressed.s {
                state.blutti.start_jump();
            }
            if just_pressed.w {
                state.blutti.start_dash();
            }
            state.blutti.update();
            for monster in state.level.monsters.iter_mut() {
                monster.update();
            }
            state.blutti.handle_effects();

            if !state.blutti.is_alive() || state.blutti.finished_level {
                state.game_state = GameState::GameOver(state.blutti.finished_level);
            } else if state.blutti.died {
                state.game_state = GameState::Died;
            }
        }
        GameState::GameOver(won) => {
            if just_pressed.n {
                if won {
                    restart(state.blutti.current_level + 1, won);
                } else {
                    restart(0, won);
                }
            }
        }
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
