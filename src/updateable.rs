use firefly_rust::{math, Point, HEIGHT};

use crate::{point_math::*, rect::*, state::*, tile_collider::*, vec2::*};

#[inline]
pub fn movement_to_step(amount: f32) -> i32 {
    if amount > 0. {
        1
    } else if amount < 0. {
        -1
    } else {
        0
    }
}

pub trait Updateable {
    fn update(&mut self);

    fn position(&self) -> Point;

    fn stop_movement(&mut self);

    fn move_horizontally(
        &mut self,
        mut position: Point,
        velocity: Vec2,
        mut remainder: Vec2,
    ) -> (Point, Vec2) {
        remainder.x += velocity.x;
        let amount = math::floor(remainder.x + 0.5);
        remainder.x -= amount;
        let step = movement_to_step(amount);
        for _ in 0..amount.abs() as i32 {
            let test_pos = position.addx(step);
            let nudge_pos = test_pos.addy(-1);
            if test_pos.is_in_screen() && !self.collision_at(test_pos) {
                position = test_pos
            } else if nudge_pos.is_in_screen() && !self.collision_at(nudge_pos) {
                // There was a collision, let's nudge up
                position = nudge_pos
            } else {
                self.stop_movement();
                break;
            }
        }

        (position, remainder)
    }

    fn move_vertically(
        &mut self,
        mut position: Point,
        velocity: Vec2,
        mut remainder: Vec2,
    ) -> (Point, Vec2) {
        remainder.y += velocity.y;
        let amount = math::floor(remainder.y + 0.5);
        remainder.y -= amount;
        let step = movement_to_step(amount);
        for _ in 0..amount.abs() as i32 {
            let test_pos = position.addy(step);
            if test_pos.y >= 0 && test_pos.y < HEIGHT && !self.collision_at(test_pos) {
                position.y += step;
            } else {
                self.stop_movement();
                break;
            }
        }

        (position, remainder)
    }

    fn collision(&self, position: Point) -> TileCollider {
        let state = get_state();
        state
            .level
            .collision_at_position(position)
            .map_or(TileCollider::None, |c| c.tile_collider)
    }

    fn rect(&self) -> Rect {
        Rect::from(self.position())
    }

    fn overlaps(&self, other: Rect) -> bool {
        self.rect().overlaps(other)
    }

    fn is_tile_empty(&self, position: Point) -> bool {
        match self.collision(position) {
            TileCollider::None
            | TileCollider::Collectable(_)
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

    fn collision_at(&self, position: Point) -> bool {
        !(self.is_tile_free(position)
            && self.is_tile_free(position.top_right())
            && self.is_tile_free(position.bottom_left())
            && self.is_tile_free(position.bottom_right()))
    }

    fn is_standing(&self) -> bool {
        !(self.is_tile_empty(self.position().below_bottom_left())
            && self.is_tile_empty(self.position().below_bottom_right()))
    }

    fn is_standing_on(&self, collision: TileCollider) -> bool {
        self.collision(self.position().below_bottom_left()) == collision
            || self.collision(self.position().below_bottom_right()) == collision
    }
}
