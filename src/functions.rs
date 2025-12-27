use firefly_rust::{get_random, math};

use crate::state::*;

#[inline]
pub fn axis_to_speed(x: i32) -> f32 {
    if x.abs() > 400 {
        1.0
    } else {
        0.5
    }
}

pub fn play_sound(sound: &str) {
    let state = get_state();
    state.fx.clear();
    state.fx.add_file(sound);
}

pub fn play_music(sound: &str) {
    let state = get_state();
    state.theme.clear();
    state.theme.add_file(sound);
}

// # Scale {value} between {min} and {max} into 0..1
///
/// ```
/// assert_eq!(scale(0, 10, 1), 0.1);
/// assert_eq!(scale(0, 100, 45), 0.45);
/// ```
pub fn scale(min: u32, max: u32, value: u32) -> f32 {
    (value as f32 - min as f32) / (max as f32 - min as f32)
}

// # Random value between 0 and {max}
pub fn random_value(max: i32) -> i32 {
    let rnd = get_random();
    math::floor(scale(u32::MIN, u32::MAX, rnd) * max as f32) as i32
}
