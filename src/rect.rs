use firefly_rust::Point;

use crate::constants::*;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Rect {
    top_left: Point,
    bottom_right: Point,
}

impl Rect {
    pub fn from(position: Point) -> Self {
        Self {
            top_left: position,
            bottom_right: Point {
                x: position.x + TILE_WIDTH - 1,
                y: position.y + TILE_HEIGHT - 1,
            },
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.top_left.x
            && point.x <= self.bottom_right.x
            && point.y >= self.top_left.y
            && point.y <= self.bottom_right.y
    }

    pub fn overlaps(&self, other: Rect) -> bool {
        self.contains(other.top_left) || self.contains(other.bottom_right)
    }
}
