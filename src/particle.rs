use firefly_rust::{Point, HEIGHT, WIDTH};

use crate::{animation::*, drawable::*, drawing::*, functions::*, state::*};

#[derive(PartialEq, Clone, Debug)]
pub enum ParticleMovement {
    Stationary,
    Falling,
    Following(i32),
}

#[derive(Clone, Debug)]
pub struct Particle {
    position: Point,
    animation: Animation,
    movement: ParticleMovement,
}

impl Particle {
    const SPEED: i32 = 1;

    pub fn new(position: Point, animation: Animation, movement: ParticleMovement) -> Self {
        Particle {
            position,
            animation,
            movement,
        }
    }

    pub fn stationary(position: Point, animation: Animation) -> Self {
        Self::new(position, animation, ParticleMovement::Stationary)
    }

    pub fn following(position: Point, animation: Animation, offset_x: i32) -> Self {
        Self::new(position, animation, ParticleMovement::Following(offset_x))
    }

    pub fn random(sprite: Sprite) -> Self {
        Particle {
            position: Point {
                x: random_value(WIDTH + 64),
                y: -3,
            },
            animation: Animation::looping([sprite, sprite + 1].into(), 15),
            movement: ParticleMovement::Falling,
        }
    }

    pub fn update(&mut self) {
        self.animation.update();
        self.update_movement();
    }

    pub fn should_be_removed(&self) -> bool {
        self.animation.finished || self.position.y > HEIGHT
    }

    fn update_movement(&mut self) {
        match self.movement {
            ParticleMovement::Falling => {
                let new_x = match random_value(100) {
                    90.. => self.position.x + Self::SPEED,
                    60..90 => self.position.x - Self::SPEED,
                    _ => self.position.x,
                };
                let new_y = match random_value(100) {
                    90.. => self.position.y - Self::SPEED,
                    40..90 => self.position.y + Self::SPEED,
                    _ => self.position.y,
                };
                self.position = Point { x: new_x, y: new_y };
            }
            ParticleMovement::Following(offset_x) => {
                let state = get_state();
                let position = state.blutti.position;
                self.position = Point {
                    x: position.x + offset_x,
                    y: position.y,
                }
            }
            ParticleMovement::Stationary => (),
        }
    }
}

impl Drawable for Particle {
    fn draw(&self) {
        draw_tile(self.animation.current_sprite(), self.position);
    }

    fn draw_debug(&self) {}
}
