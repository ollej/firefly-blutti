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
    velocity: Vec2,
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
    velocity: Vec2,
}

impl From<MonsterSerde> for Monster {
    fn from(value: MonsterSerde) -> Monster {
        Monster {
            position: value.position,
            velocity: value.velocity,
            remainder: Vec2::zero(),
            sprite: value.sprite,
            animation: Monster::animation_from(value.velocity, value.sprite),
        }
    }
}

impl Monster {
    fn animation_from(velocity: Vec2, sprite: Sprite) -> Animation {
        if velocity.x > 0.0 {
            Animation::looping([sprite + 2, sprite + 3], 10)
        } else {
            Animation::looping([sprite, sprite + 1], 10)
        }
    }

    fn change_direction_x(&mut self) {
        self.velocity.x *= -1.0;
        self.animation = Self::animation_from(self.velocity, self.sprite);
    }

    fn change_direction_y(&mut self) {
        self.velocity.y *= -1.0;
        self.animation = Self::animation_from(self.velocity, self.sprite);
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
            self.move_horizontally(self.position, self.velocity, self.remainder);

        // Move y position
        (self.position, self.remainder) =
            self.move_vertically(self.position, self.velocity, self.remainder);
    }

    fn collision_at(&self, position: Point) -> bool {
        // Handle direction change at edge of platforms
        let collision_below = if self.velocity.x < 0.0 {
            self.is_tile_free(position.below_bottom_left())
        } else if self.velocity.x > 0.0 {
            self.is_tile_free(position.below_bottom_right())
        } else {
            false
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
        if self.velocity.x != 0.0 {
            self.change_direction_x();
        } else {
            self.change_direction_y();
        }
    }
}
