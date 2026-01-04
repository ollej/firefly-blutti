use firefly_rust::Point;

use crate::constants::*;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Rect {
    position: Point,
    width: i32,
    height: i32,
}

impl Rect {
    pub fn from(position: Point) -> Self {
        Self {
            position,
            width: TILE_WIDTH,
            height: TILE_HEIGHT,
        }
    }

    pub fn from_width_and_height(position: Point, width: i32, height: i32) -> Self {
        Self {
            position,
            width,
            height,
        }
    }

    pub fn top_left(&self) -> Point {
        self.position
    }

    pub fn top_right(&self) -> Point {
        Point {
            x: self.position.x + self.width - 1,
            y: self.position.y,
        }
    }

    pub fn bottom_right(&self) -> Point {
        Point {
            x: self.position.x + self.width - 1,
            y: self.position.y + self.height - 1,
        }
    }

    pub fn bottom_left(&self) -> Point {
        Point {
            x: self.position.x,
            y: self.position.y + self.height - 1,
        }
    }

    pub fn below_bottom_left(&self) -> Point {
        Point {
            x: self.position.x,
            y: self.position.y + self.height,
        }
    }

    pub fn below_bottom_middle(&self) -> Point {
        Point {
            x: self.position.x,
            y: self.position.y + self.height / 2,
        }
    }

    pub fn below_bottom_right(&self) -> Point {
        Point {
            x: self.position.x + self.width - 1,
            y: self.position.y + self.height,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.position.x
            && point.x <= self.bottom_right().x
            && point.y >= self.position.y
            && point.y <= self.bottom_right().y
    }

    pub fn overlaps(&self, other: Rect) -> bool {
        self.contains(other.position) || self.contains(other.bottom_right())
    }
}
