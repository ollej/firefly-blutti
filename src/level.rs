extern crate alloc;

use alloc::vec::Vec;
use firefly_rust::{clear_screen, load_file_buf, Color, Point};
use serde::Deserialize;

use crate::{
    blutti::*, collision::*, constants::*, drawable::*, drawing::*, functions::*, game_state::*,
    monster::*, particle::*, point_math::*, rect::*, serde::*, state::*, updateable::*,
};

pub type LevelNumber = i32;

#[derive(Clone, Debug, Deserialize)]
pub struct Level {
    tiles: Vec<Sprite>,
    #[serde(with = "ColorDef")]
    background_color: Color,
    #[serde(with = "ColorDef")]
    pub font_color: Color,
    pub stars: i32,
    #[serde(with = "PointDef")]
    pub start_position: Point,
    particle_chance: i32,
    particle_sprite: i32,
    pub monsters: Vec<Monster>,
    #[serde(skip)]
    pub particles: Vec<Particle>,
    #[serde(skip)]
    original_monsters: Vec<Monster>,
}

impl Level {
    const START_LEVEL: LevelNumber = 1;
    pub const MIN: Point = Point::MIN;
    pub const MAX: Point = Point {
        x: Point::MAX.x - TILE_WIDTH,
        y: Point::MAX.y - TILE_WIDTH,
    };

    pub fn load_level(level: LevelNumber) -> Self {
        let level_name = LEVELS[level as usize];
        let level_data = load_file_buf(level_name).expect("Couldn't load level data");
        let mut level =
            serde_json::from_slice::<Level>(level_data.data()).expect("Couldn't parse level data");
        level.original_monsters = level.monsters.clone();
        level
    }

    pub fn restart(mut level: LevelNumber, won: bool) -> LevelNumber {
        let state = get_state();
        if level >= LEVELS.len() as LevelNumber {
            // Restart at level 1, as level 0 is a debug level
            level = Self::START_LEVEL;
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

    pub fn update(&mut self) {
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

    pub fn draw(&mut self) {
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

    pub fn draw_children(&self) {
        for monster in self.monsters.iter() {
            monster.draw();
        }
        for particle in self.particles.iter() {
            particle.draw();
        }
    }

    pub fn reset(&mut self) {
        self.monsters = self.original_monsters.clone();
    }

    pub fn collision_at_position(&self, position: Point) -> Option<Collision> {
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

    pub fn all_collisions_at_rect(&self, position: Point) -> [Option<Collision>; 4] {
        [
            self.collision_at_position(position),
            self.collision_at_position(position.top_right()),
            self.collision_at_position(position.bottom_left()),
            self.collision_at_position(position.bottom_right()),
        ]
    }

    pub fn remove_tile(&mut self, position: Point) {
        let tile_pos = get_tile_index(position);
        self.tiles[tile_pos as usize] = 0;
    }

    pub fn monsters_at_position(&self, position: Point) -> Vec<&Monster> {
        self.monsters
            .iter()
            .filter(|monster| monster.rect().contains(position))
            .collect()
    }

    pub fn deadly_monsters_overlapping_rect(&self, rect: Rect) -> bool {
        self.monsters
            .iter()
            .any(|monster| monster.collision == MonsterCollision::Deadly && monster.overlaps(rect))
    }

    fn sprite_at_position(&self, point: Point) -> Sprite {
        let tile_pos = get_tile_index(point);
        let maybe_sprite = self.tiles.get(tile_pos as usize);
        // The default sprite should be a sprite with TileCollider::Full
        // in the COLLISION list. Otherwise, if the Blutti is standing
        // on the bottom edge of the screen, it will be considered falling.
        maybe_sprite.unwrap_or(&4) - 1
    }
}
