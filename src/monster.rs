extern crate alloc;

use alloc::vec::Vec;
use fixedstr::{str128, str_format};
use serde::Deserialize;

use firefly_rust::Point;

use crate::{
    animation::*, constants::*, drawable::*, drawing::*, point_math::*, rect::*, serde::*,
    state::*, updateable::*, vec2::*,
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
    BlockingMonster,
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
    frames: Sprite,
    gravity: bool,
    movement: MonsterMovement,
    #[serde(with = "PointDef")]
    position: Point,
    reverse_sprites: Vec<Sprite>,
    sprites: Vec<Sprite>,
    velocity: Vec2,
    width: Option<i32>,
    height: Option<i32>,
}

impl From<MonsterSerde> for Monster {
    // FIXME: Cache both animations
    fn from(value: MonsterSerde) -> Monster {
        Monster {
            collision: value.collision,
            gravity: value.gravity,
            movement: value.movement,
            position: value.position,
            remainder: Vec2::zero(),
            velocity: value.velocity,
            width: value.width.unwrap_or(TILE_WIDTH),
            height: value.height.unwrap_or(TILE_HEIGHT),
            animations: Monster::animations_from(value.sprites, value.frames),
            reverse_animations: Monster::animations_from(value.reverse_sprites, value.frames),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(from = "MonsterSerde")]
pub struct Monster {
    #[serde(skip)]
    animations: Animations,
    #[serde(skip)]
    reverse_animations: Animations,
    pub collision: MonsterCollision,
    gravity: bool,
    movement: MonsterMovement,
    #[serde(with = "PointDef")]
    position: Point,
    #[serde(skip)]
    remainder: Vec2,
    pub velocity: Vec2,
    width: i32,
    height: i32,
}

impl Monster {
    const TIME_PER_FRAME: i32 = 10;

    fn animations_from(sprites: Vec<Sprite>, frames: i32) -> Animations {
        let animations = sprites
            .iter()
            .map(|sprite| {
                let sprites: Vec<i32> = (0..frames).map(|offset| sprite + offset).collect();
                Animation::looping(sprites, Self::TIME_PER_FRAME)
            })
            .collect();
        Animations::new(animations)
    }

    fn rect_from_position(&self, position: Point) -> Rect {
        Rect::from_width_and_height(position, self.width, self.height)
    }

    fn change_direction_x(&mut self) {
        self.velocity.x *= -1.0;
        self.animations.reset();
    }

    fn change_direction_y(&mut self) {
        if !self.gravity {
            self.velocity.y *= -1.0;
            self.reverse_animations.reset();
        }
    }
}

impl Drawable for Monster {
    fn draw(&self) {
        let animations = if self.velocity.x > 0.0 || self.velocity.y > 0.0 {
            &self.animations
        } else {
            &self.reverse_animations
        };
        let tiles_w = self.width / TILE_WIDTH;
        let tiles_h = self.height / TILE_HEIGHT;
        let mut index = 0;
        let sprites = animations.current_sprites();
        for x in 0..tiles_w {
            for y in 0..tiles_h {
                let position = self.position.addx(x * TILE_WIDTH).addy(y * TILE_HEIGHT);
                sprites
                    .get(index)
                    .map(|sprite| draw_tile(*sprite, position));
                index += 1;
            }
        }
    }

    fn draw_debug(&self) {}
}

impl Updateable for Monster {
    fn position(&self) -> Point {
        self.position
    }

    fn rect(&self) -> Rect {
        self.rect_from_position(self.position)
    }

    fn update(&mut self) {
        self.animations.update();
        self.reverse_animations.update();

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
        let rect = self.rect_from_position(position);

        // Handle direction change at edge of platforms
        let collision_below = match self.movement {
            MonsterMovement::TurnsAtEdge => {
                if self.velocity.x < 0.0 {
                    self.is_position_free(rect.below_bottom_left())
                } else if self.velocity.x > 0.0 {
                    self.is_position_free(rect.below_bottom_right())
                } else {
                    false
                }
            }
            MonsterMovement::Flying | MonsterMovement::FollowsPlayer | MonsterMovement::Moving => {
                false
            }
        };

        !(self.is_position_free(rect.top_left())
            && self.is_position_free(rect.top_right())
            && self.is_position_free(rect.bottom_left())
            && self.is_position_free(rect.bottom_right())
            && !collision_below)
    }

    fn is_monster_blocking(&self, monster: &Monster) -> bool {
        matches!(
            monster.collision,
            MonsterCollision::Blocking | MonsterCollision::BlockingMonster
        )
    }

    fn stop_movement(&mut self, stop_direction: StopDirection) {
        if stop_direction == StopDirection::X {
            self.change_direction_x();
        } else {
            self.change_direction_y();
        }
    }
}
