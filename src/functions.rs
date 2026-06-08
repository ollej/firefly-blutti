use firefly_rust::get_random;

use crate::state::*;

#[inline]
pub fn axis_to_speed(x: i32) -> f32 {
    if x.abs() > 400 { 1.0 } else { 0.5 }
}

pub fn play_sound(sound: &'static str) {
    let state = get_state();
    state.fx.clear();
    state.fx.add_file(sound);
}

pub fn play_music(sound: &'static str) {
    let state = get_state();
    state.theme.clear();
    state.theme.add_file(sound);
}

/// Get a random integer between 0 and `max`.
pub fn random_value(max: i32) -> i32 {
    (get_random() as i32).abs() % max
}
