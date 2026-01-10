use firefly_rust::Point;
use serde::Deserialize;

#[derive(Copy, Clone, PartialEq, Default, Debug, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn is_zero(&self) -> bool {
        self.x.abs() == 0.0 && self.y.abs() == 0.0
    }

    pub fn add(&self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<Point> for Vec2 {
    fn from(value: Point) -> Self {
        Vec2::new(value.x as f32, value.y as f32)
    }
}
