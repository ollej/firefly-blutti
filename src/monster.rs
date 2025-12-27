use firefly_rust::Point;

use serde::Deserialize;

use crate::{
    animation::*, drawable::*, drawing::*, point_math::*, serde::*, updateable::*, vec2::*,
};

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(from = "MonsterSerde")]
pub struct Monster {
    #[serde(with = "PointDef")]
    position: Point,
    sprite: i32,
    movement: f32,
    #[serde(skip)]
    remainder: Vec2,
    #[serde(skip)]
    animation: Animation,
}

#[derive(Deserialize)]
struct MonsterSerde {
    #[serde(with = "PointDef")]
    position: Point,
    sprite: i32,
    movement: f32,
}

impl From<MonsterSerde> for Monster {
    fn from(value: MonsterSerde) -> Monster {
        Monster {
            position: value.position,
            movement: value.movement,
            remainder: Vec2::zero(),
            sprite: value.sprite,
            animation: Monster::animation_from(value.movement, value.sprite),
        }
    }
}

impl Monster {
    fn animation_from(movement: f32, sprite: i32) -> Animation {
        if movement >= 0.0 {
            Animation::looping([sprite + 2, sprite + 3], 10)
        } else {
            Animation::looping([sprite, sprite + 1], 10)
        }
    }

    fn change_direction(&mut self) {
        self.movement *= -1.0;
        self.animation = Self::animation_from(self.movement, self.sprite);
    }

    fn velocity(&self) -> Vec2 {
        Vec2 {
            x: self.movement,
            y: 0.0,
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

        // Move X position
        (self.position, self.remainder) =
            self.move_horizontally(self.position, self.velocity(), self.remainder);

        // Move y position
        (self.position, self.remainder) =
            self.move_vertically(self.position, self.velocity(), self.remainder);
    }

    fn collision_at(&self, position: Point) -> bool {
        let collision_below = if self.movement < 0.0 {
            self.is_tile_free(position.below_bottom_left())
        } else {
            self.is_tile_free(position.below_bottom_right())
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

    fn stop_movement(&mut self) {
        self.change_direction();
    }
}
