use firefly_rust::{Color, Point};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(remote = "Point")]
pub struct PointDef {
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
#[serde(remote = "Color")]
pub enum ColorDef {
    None,
    Black,
    Purple,
    Red,
    Orange,
    Yellow,
    LightGreen,
    Green,
    DarkGreen,
    DarkBlue,
    Blue,
    LightBlue,
    Cyan,
    White,
    LightGray,
    Gray,
    DarkGray,
}
