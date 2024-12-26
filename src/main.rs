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
    0, 0, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 0, 0, 10, 0, 10, 0, 0, 10, 0, 10, 0, 0, 0,
    0, 4, 3, 3, 5, 0, 0, 0, 0, 0, 0, 4, 3, 3, 5, 0, 0, 0, 4, 3, 3, 3, 3, 9, 3, 3, 3, 5, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 4, 3, 3, 3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 10, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 4, 3, 3, 3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 11, 0, 0,
    3, 3, 3, 3, 8, 8, 8, 8, 8, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
];

struct Level {
    tiles: [i32; 600],
    stars: i32,
    collision: [TileCollider; 64],
    start_position: Point,
    monsters: [Monster; 2],
}

impl Level {
    fn new(tiles: [i32; 600], stars: i32) -> Self {
        let mut collision = [TileCollider::None; 64];
        collision[3] = TileCollider::Full;
        collision[4] = TileCollider::Full;
        collision[5] = TileCollider::Full;
        collision[6] = TileCollider::Full;
        collision[7] = TileCollider::Full;
        collision[8] = TileCollider::Full;
        collision[9] = TileCollider::Climbable;
        collision[10] = TileCollider::Collectable;
        collision[11] = TileCollider::Collectable;
        collision[12] = TileCollider::Collectable;
        let start_position = Point { x: 8, y: 144 };
        let monsters = [Monster::alive(Point { x: 120, y: 144 }), Monster::dead()];
        Level {
            tiles,
            stars,
            collision,
            start_position,
            monsters,
        }
    }

    fn tile_at_pos(&self, tile_pos: usize) -> i32 {
        self.tiles[tile_pos]
    }

    fn tile_at_point(&self, point: Point) -> i32 {
        let tile_pos = get_tile_index(point);
        self.tile_at_pos(tile_pos)
    }

    fn collision_at_point(&self, test_point: Point) -> TileCollider {
        //log_debug(str_format!(str256, "x: {}", test_point.x).as_str());
        //log_debug(str_format!(str256, "y: {}", test_point.y).as_str());
        //log_debug(str_format!(str256, "tile_pos: {}", tile_pos).as_str());
        //log_debug(str_format!(str256, "tile: {}", tile).as_str());
        let tile = self.tile_at_point(test_point);
        self.collision[tile as usize]
    }

    fn collectable_at_point(&self, position: Point) -> bool {
        self.collision_at_point(position) == TileCollider::Collectable
    }

    fn collect_item(&mut self, position: Point) -> Option<i32> {
        if self.collectable_at_point(position) {
            let tile_pos = get_tile_index(position);
            let collected_tile = self.tiles[tile_pos];
            self.tiles[tile_pos] = 0;
            Some(collected_tile)
        } else {
            None
        }
    }
}

static mut STATE: OnceCell<State> = OnceCell::new();

struct State {
    blutti: Blutti,
    spritesheet: FileBuf,
    font: FileBuf,
    fx: audio::Node<audio::Gain>,
    theme: audio::Node<audio::Gain>,
    level: Level,
    game_state: GameState,
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
    Climbable,
    Collectable,
    None,
}

enum GameState {
    Playing,
    Menu,
    GameOver(bool),
}

trait Drawable {
    fn draw(&self);
    fn position(&self) -> Point;

    fn collision(&self, position: Point) -> TileCollider {
        let state = get_state();
        state.level.collision_at_point(position)
    }

    fn is_tile_empty(&self, position: Point) -> bool {
        let tile = self.collision(position);
        tile == TileCollider::None || tile == TileCollider::Collectable
    }

    fn is_tile_free(&self, position: Point) -> bool {
        self.collision(position) != TileCollider::Full
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
}

trait Updateable {
    fn update(&mut self);
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
    finished_level: bool,
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
            finished_level: false,
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

    fn position(&self) -> Point {
        self.position
    }
}

impl Updateable for Blutti {
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
}

impl Blutti {
    const SIZE: i32 = 8;
    const SPEED: i32 = 2;
    const JUMP_TIME: i32 = 8;
    const JUMP_SPEED: i32 = 2;
    const DASH_TIME: i32 = 8;
    const DASH_WAIT_TIME: i32 = 32;
    const DASH_SPEED: i32 = 3;
    const GRAVITY: i32 = 2;
    const MAX_FALL_HEIGHT: i32 = 30;
    const START_POSITION: Point = Point {
        x: WIDTH / 2 - Self::SIZE,
        y: HEIGHT - Self::SIZE - TILE_HEIGHT,
    };
    const MIN: Point = Point::MIN;
    const MAX: Point = Point {
        x: Point::MAX.x - Self::SIZE,
        y: Point::MAX.y - Self::SIZE,
    };

    fn with_start_position(start_position: Point) -> Self {
        Blutti {
            position: start_position,
            start_position,
            ..Blutti::default()
        }
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
            play_sound("sound_jump");
            self.jump_timer = Self::JUMP_TIME;
        }
    }

    fn start_dash(&mut self) {
        if self.dash_timer == 0 {
            play_sound("sound_dash");
            self.dash_timer = Self::DASH_TIME;
        }
    }

    fn handle_effects(&mut self) {
        match self.collision(self.position) {
            TileCollider::Collectable => self.collect_item(),
            _ => (),
        }
    }

    fn collect_item(&mut self) {
        let state = get_state();
        if state.level.collectable_at_point(self.position) {
            let tile = state.level.tile_at_point(self.position);
            match tile {
                10 => {
                    self.points += 1;
                    self.stars += 1;
                    state.level.collect_item(self.position);
                    play_sound("sound_coin");
                }
                11 => {
                    self.lives += 1;
                    state.level.collect_item(self.position);
                    play_sound("sound_powerup");
                }
                12 => {
                    if self.stars >= state.level.stars {
                        self.finished_level = true;
                        play_sound("sound_exit");
                    } else {
                        play_sound("sound_wrong");
                    }
                }
                _ => (),
            }
        }
    }

    fn die(&mut self) {
        self.lives -= 1;
        self.reset();
        play_sound("sound_death");
    }

    fn reset(&mut self) {
        self.position = self.start_position;
        self.jump_timer = 0;
        self.dash_timer = 0;
        self.fall_timer = 0;
        self.movement = 0;
        self.direction = Direction::Left;
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

struct Monster {
    position: Point,
    alive: bool,
    tile: i32,
}

impl Monster {
    fn alive(position: Point) -> Self {
        Monster {
            position,
            alive: true,
            tile: 13,
        }
    }

    fn dead() -> Self {
        Monster {
            position: Point::default(),
            alive: false,
            tile: 0,
        }
    }
}

impl Drawable for Monster {
    fn draw(&self) {
        if self.alive {
            draw_tile(self.tile, self.position());
        }
    }

    fn position(&self) -> Point {
        self.position
    }
}

impl Updateable for Monster {
    fn update(&mut self) {}
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
    let font = state.font.as_font();
    draw_text(text, &font, position, Color::Black);
}

fn render_menu() {
    display_text(
        "Press <X> to start!",
        Point {
            x: WIDTH / 2 - 38,
            y: HEIGHT / 2 - 3,
        },
    );
}

fn render_gameover(won: bool) {
    if won {
        display_text(
            "You win!",
            Point {
                x: WIDTH / 2 - 16,
                y: HEIGHT / 2 - 3,
            },
        );
    } else {
        display_text(
            "Game Over!",
            Point {
                x: WIDTH / 2 - 20,
                y: HEIGHT / 2 - 3,
            },
        );
    }
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

fn render_monsters() {
    let state = get_state();
    for monster in state.level.monsters.iter() {
        monster.draw();
    }
}

fn render_level() {
    let state = get_state();
    for (i, tile) in state.level.tiles.iter().enumerate() {
        let point = Point {
            x: ((i as i32 % TILES_H) * TILE_WIDTH),
            y: ((i as i32 / TILES_H) * TILE_HEIGHT),
        };
        draw_tile(*tile, point);
    }
}

#[no_mangle]
extern "C" fn boot() {
    let fx = audio::OUT.add_gain(1.0);
    let theme = audio::OUT.add_gain(0.5);
    let level = Level::new(LEVEL.clone(), 10);
    let state = State {
        blutti: Blutti::with_start_position(level.start_position),
        spritesheet: load_file_buf("spritesheet").unwrap(),
        font: load_file_buf("font").unwrap(),
        fx,
        theme,
        level,
        game_state: GameState::Menu,
    };
    unsafe { STATE.set(state) }.ok().unwrap();
    play_music("sound_theme");
}

#[no_mangle]
extern "C" fn update() {
    let state = get_state();
    let buttons = read_buttons(Peer::COMBINED);
    match state.game_state {
        GameState::Menu => {
            if buttons.s {
                state.game_state = GameState::Playing;
            }
        }
        GameState::Playing => {
            if !state.blutti.is_alive() || state.blutti.finished_level {
                log_debug("unalived");
                state.game_state = GameState::GameOver(state.blutti.finished_level);
            } else {
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
                if buttons.s {
                    state.blutti.start_jump();
                }
                if buttons.w {
                    state.blutti.start_dash();
                }
                state.blutti.update();
                for monster in state.level.monsters.iter_mut() {
                    monster.update();
                }
                state.blutti.handle_effects();
            }
        }
        GameState::GameOver(_won) => {
            if buttons.s {
                state.level = Level::new(LEVEL, 10);
                state.blutti = Blutti::with_start_position(state.level.start_position);
                state.game_state = GameState::Menu;
            }
        }
    }
}

#[no_mangle]
extern "C" fn render() {
    let state = get_state();
    clear_screen(Color::White);
    match state.game_state {
        GameState::Menu => {
            render_level();
            render_ui();
            render_menu();
        }
        GameState::Playing => {
            render_level();
            state.blutti.draw();
            render_monsters();
            render_ui();
        }
        GameState::GameOver(won) => {
            render_level();
            render_ui();
            render_gameover(won);
        }
    }
}
