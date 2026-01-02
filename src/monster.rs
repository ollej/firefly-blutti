extern crate alloc;

use alloc::vec::Vec;
use fixedstr::{str128, str_format};
use serde::Deserialize;

use firefly_rust::{log_debug, Point};

use crate::{
    animation::*, constants::*, drawable::*, drawing::*, point_math::*, serde::*, updateable::*,
    vec2::*,
};

#[derive(Clone, Debug, Deserialize)]
pub enum MonsterMovement {
    TurnsAtEdge,
    FollowsPlayer,
    Moving,
}

impl Default for MonsterMovement {
    fn default() -> Self {
        MonsterMovement::TurnsAtEdge
    }
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub enum MonsterCollision {
    Blocking,
    Deadly,
    None,
}

impl Default for MonsterCollision {
    fn default() -> Self {
        MonsterCollision::Deadly
    }
}

#[derive(Deserialize)]
struct MonsterSerde {
    collision: MonsterCollision,
    gravity: bool,
    movement: MonsterMovement,
    #[serde(with = "PointDef")]
    position: Point,
    sprite: i32,
    sprites: i32,
    velocity: Vec2,
}

impl From<MonsterSerde> for Monster {
    fn from(value: MonsterSerde) -> Monster {
        Monster {
            collision: value.collision,
            gravity: value.gravity,
            movement: value.movement,
            position: value.position,
            velocity: value.velocity,
            remainder: Vec2::zero(),
            sprite: value.sprite,
            sprites: value.sprites,
            animation: Monster::animation_from(value.velocity, value.sprite, value.sprites),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(from = "MonsterSerde")]
pub struct Monster {
    #[serde(skip)]
    animation: Animation,
    pub collision: MonsterCollision,
    gravity: bool,
    movement: MonsterMovement,
    #[serde(with = "PointDef")]
    position: Point,
    #[serde(skip)]
    remainder: Vec2,
    sprite: i32,
    sprites: i32,
    velocity: Vec2,
}

impl Monster {
    fn animation_from(velocity: Vec2, sprite: Sprite, sprite_count: i32) -> Animation {
        if velocity.x > 0.0 || velocity.y > 0.0 {
            let sprites: Vec<i32> = (0..sprite_count)
                .map(|offset| sprite + sprite_count + offset)
                .collect();
            Animation::looping(sprites, 10)
        } else {
            let sprites: Vec<i32> = (0..sprite_count).map(|offset| sprite + offset).collect();
            Animation::looping(sprites, 10)
        }
    }

    fn change_direction_x(&mut self) {
        self.velocity.x *= -1.0;
        self.animation = Self::animation_from(self.velocity, self.sprite, self.sprites);
    }

    fn change_direction_y(&mut self) {
        if !self.gravity {
            self.velocity.y *= -1.0;
            self.animation = Self::animation_from(self.velocity, self.sprite, self.sprites);
        }
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

        // Apply gravity
        if self.gravity == true {
            self.velocity.y = (self.velocity.y + GRAVITY_ACCELERATION).min(GRAVITY_MAX);
            log_debug(
                str_format!(
                    str128,
                    "gravity velocity x: {}, y: {}",
                    self.velocity.x,
                    self.velocity.y
                )
                .as_str(),
            );
        }

        // Move X position
        (self.position, self.remainder) =
            self.move_horizontally(self.position, self.velocity, self.remainder);

        // Move y position
        (self.position, self.remainder) =
            self.move_vertically(self.position, self.velocity, self.remainder);
    }

    fn collision_at(&self, position: Point) -> bool {
        // Handle direction change at edge of platforms
        let collision_below = match self.movement {
            MonsterMovement::TurnsAtEdge => {
                if self.velocity.x < 0.0 {
                    self.is_tile_free(position.below_bottom_left())
                } else if self.velocity.x > 0.0 {
                    self.is_tile_free(position.below_bottom_right())
                } else {
                    false
                }
            }
            MonsterMovement::FollowsPlayer | MonsterMovement::Moving => false,
        };

        !(self.is_tile_free(position)
            && self.is_tile_free(position.top_right())
            && self.is_tile_free(position.bottom_left())
            && self.is_tile_free(position.bottom_right())
            && !collision_below)
    }

    fn position(&self) -> Point {
        self.position
    }

    fn stop_movement(&mut self, stop_direction: StopDirection) {
        if stop_direction == StopDirection::X {
            self.change_direction_x();
        } else {
            self.change_direction_y();
        }
    }
}
