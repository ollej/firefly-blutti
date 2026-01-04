use firefly_rust::Point;

use crate::{constants::*, level::*};

pub trait PointMath {
    fn top_right(&self) -> Point;
    #[allow(dead_code)]
    fn top_middle(&self) -> Point;
    fn bottom_left(&self) -> Point;
    #[allow(dead_code)]
    fn bottom_middle(&self) -> Point;
    fn bottom_right(&self) -> Point;
    fn below_bottom_left(&self) -> Point;
    #[allow(dead_code)]
    fn below_bottom_middle(&self) -> Point;
    fn below_bottom_right(&self) -> Point;
    fn addx(&self, addend: i32) -> Point;
    fn addy(&self, addend: i32) -> Point;
    fn is_in_screen(&self) -> bool;
}

impl PointMath for Point {
    fn top_right(&self) -> Point {
        Point {
            x: self.x + TILE_WIDTH - 1,
            y: self.y,
        }
    }

    fn top_middle(&self) -> Point {
        Point {
            x: self.x + TILE_WIDTH / 2,
            y: self.y,
        }
    }

    fn bottom_left(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + TILE_HEIGHT - 1,
        }
    }

    fn bottom_middle(&self) -> Point {
        Point {
            x: self.x + TILE_WIDTH / 2,
            y: self.y + TILE_HEIGHT - 1,
        }
    }

    fn bottom_right(&self) -> Point {
        Point {
            x: self.x + TILE_WIDTH - 1,
            y: self.y + TILE_HEIGHT - 1,
        }
    }

    fn below_bottom_left(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + TILE_HEIGHT,
        }
    }

    fn below_bottom_middle(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + TILE_HEIGHT / 2,
        }
    }

    fn below_bottom_right(&self) -> Point {
        Point {
            x: self.x + TILE_WIDTH - 1,
            y: self.y + TILE_HEIGHT,
        }
    }

    fn addx(&self, addend: i32) -> Point {
        Point {
            x: self.x + addend,
            y: self.y,
        }
    }

    fn addy(&self, addend: i32) -> Point {
        Point {
            x: self.x,
            y: self.y + addend,
        }
    }

    fn is_in_screen(&self) -> bool {
        self.x >= Level::MIN.x
            && self.x <= Level::MAX.x
            && self.y >= Level::MIN.y
            && self.y <= Level::MAX.y
    }
}
