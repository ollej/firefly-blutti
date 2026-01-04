extern crate alloc;

use alloc::vec::Vec;
use serde::Deserialize;

use firefly_rust::{log_debug, Point};

use crate::{
    animation::*, constants::*, drawable::*, drawing::*, point_math::*, serde::*, state::*,
    updateable::*, vec2::*,
};

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub enum MonsterMovement {
    Flying,
    FollowsPlayer,
    Moving,
    TurnsAtEdge,
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
    reverse_sprite: i32,
    sprite: Sprite,
    frames: Sprite,
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
            reverse_sprite: value.reverse_sprite,
            sprite: value.sprite,
            frames: value.frames,
            animation: Monster::animation_from(
                value.velocity,
                value.sprite,
                value.reverse_sprite,
                value.frames,
            ),
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
    reverse_sprite: Sprite,
    sprite: Sprite,
    frames: i32,
    pub velocity: Vec2,
}

impl Monster {
    fn animation_from(
        velocity: Vec2,
        sprite: Sprite,
        reverse_sprite: Sprite,
        frames: i32,
    ) -> Animation {
        let selected_sprite = if velocity.x > 0.0 || velocity.y > 0.0 {
            reverse_sprite
        } else {
            sprite
        };
        let sprites: Vec<i32> = (0..frames).map(|offset| selected_sprite + offset).collect();
        Animation::looping(sprites, 10)
    }

    fn change_direction_x(&mut self) {
        self.velocity.x *= -1.0;
        self.animation =
            Self::animation_from(self.velocity, self.sprite, self.reverse_sprite, self.frames);
    }

    fn change_direction_y(&mut self) {
        if !self.gravity {
            self.velocity.y *= -1.0;
            self.animation =
                Self::animation_from(self.velocity, self.sprite, self.reverse_sprite, self.frames);
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
        }

        // Move X position
        if self.movement == MonsterMovement::Flying || self.is_standing() {
            (self.position, self.remainder) =
                self.move_horizontally(self.position, self.velocity, self.remainder);
        }

        // Move y position
        (self.position, self.remainder) =
            self.move_vertically(self.position, self.velocity, self.remainder);

        // Move player
        if self.collision == MonsterCollision::Blocking {
            let state = get_state();
            if state.blutti.is_standing_on_rect(self.rect()) {
                state.blutti.force_move(self.velocity);
            }
        }
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
            MonsterMovement::Flying | MonsterMovement::FollowsPlayer | MonsterMovement::Moving => {
                false
            }
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
