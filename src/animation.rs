extern crate alloc;

use alloc::vec::Vec;

use crate::{constants::*, drawing::*};

pub type Frame = i32;

#[derive(Clone, Default, Debug)]
pub struct Animations {
    pub animations: Vec<Animation>,
}

impl Animations {
    pub fn new(animations: Vec<Animation>) -> Self {
        Animations { animations }
    }

    pub fn update(&mut self) {
        self.animations
            .iter_mut()
            .for_each(|animation| animation.update());
    }

    pub fn reset(&mut self) {
        self.animations
            .iter_mut()
            .for_each(|animation| animation.reset());
    }

    pub fn current_sprites(&self) -> Vec<Sprite> {
        self.animations
            .iter()
            .map(|animation| animation.current_sprite())
            .collect()
    }
}

#[derive(Clone, Default, Debug)]
pub struct Animation {
    sprites: Vec<Sprite>,
    pub current_frame: Frame,
    time_per_frame: i32,
    frame_timer: i32,
    looping: bool,
    pub finished: bool,
}

impl Animation {
    pub fn new(sprites: Vec<Sprite>, time_per_frame: i32, looping: bool) -> Self {
        Self {
            sprites,
            current_frame: 0,
            time_per_frame,
            frame_timer: 0,
            looping,
            finished: false,
        }
    }

    pub fn animation_idle_left() -> Animation {
        Animation::looping(BLUTTI_IDLE_LEFT_SPRITES.into(), 10)
    }

    pub fn animation_idle_right() -> Animation {
        Animation::looping(BLUTTI_IDLE_RIGHT_SPRITES.into(), 10)
    }

    pub fn animation_running_left() -> Animation {
        Animation::looping(BLUTTI_LEFT_SPRITES.into(), 10)
    }

    pub fn animation_running_right() -> Animation {
        Animation::looping(BLUTTI_RIGHT_SPRITES.into(), 10)
    }

    pub fn animation_climb_left() -> Animation {
        Animation::looping(BLUTTI_CLIMB_LEFT_SPRITES.into(), 10)
    }

    pub fn animation_climb_right() -> Animation {
        Animation::looping(BLUTTI_CLIMB_RIGHT_SPRITES.into(), 10)
    }

    pub fn animation_death() -> Animation {
        Animation::once(BLUTTI_DEATH_SPRITES.into(), 5)
    }

    pub fn animation_exit_left() -> Animation {
        Animation::once(BLUTTI_EXIT_LEFT_SPRITES.into(), 5)
    }

    pub fn animation_exit_right() -> Animation {
        Animation::once(BLUTTI_EXIT_RIGHT_SPRITES.into(), 5)
    }

    pub fn once(sprites: Vec<Sprite>, time_per_frame: i32) -> Self {
        Self::new(sprites, time_per_frame, false)
    }

    pub fn looping(sprites: Vec<Sprite>, time_per_frame: i32) -> Self {
        Self::new(sprites, time_per_frame, true)
    }

    pub fn current_sprite(&self) -> Sprite {
        self.sprites[self.current_frame as usize]
    }

    pub fn update(&mut self) {
        self.frame_timer += 1;
        if self.frame_timer > self.time_per_frame {
            self.next_frame();
        }
    }

    pub fn reset(&mut self) {
        self.frame_timer = 0;
        self.current_frame = 0;
        self.finished = false;
    }

    fn next_frame(&mut self) {
        self.frame_timer = 0;
        self.current_frame += 1;
        if self.current_frame >= self.sprites.len() as Frame {
            self.current_frame = 0;
            if !self.looping {
                self.finished = true;
            }
        }
    }
}
