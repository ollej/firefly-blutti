extern crate alloc;
use alloc::vec::Vec;
use firefly_rust::{add_progress, get_me, Point, HEIGHT, WIDTH};
use fixedstr::{str32, str_format};

use crate::{
    animation::*, collision::*, constants::*, direction::*, drawable::*, drawing::*, functions::*,
    level::*, particle::*, player_state::*, point_math::*, state::*, tile_collider::*,
    updateable::*, vec2::*,
};

pub struct Blutti {
    pub position: Point,
    start_position: Point,
    jump_timer: i32,
    dash_timer: i32,
    fall_timer: i32,
    stop_timer: i32,
    jump_max_time: i32,
    jump_buffer_timer: i32,
    direction_x: DirectionX,
    direction_y: DirectionY,
    state: PlayerState,
    velocity: Vec2,
    remainder: Vec2,
    movement_modifier: f32,
    pub points: i32,
    stars: i32,
    pub lives: i32,
    pub iddqd: bool,
    pub died: bool,
    pub finished_level: bool,
    pub current_level: LevelNumber,
    current_tile: i32,
    pub animation: Animation,
    debug: bool,
}

impl Blutti {
    const RUNNING_ACCELERATION: f32 = 0.5;
    const RUNNING_STOP_TIME: i32 = 4;
    const MAX_VELOCITY: f32 = 2.0;
    const FALLING_X_ACCELERATION: f32 = 0.1;
    const MAX_FALLING_VELOCITY: f32 = 0.8;
    const JUMP_ACCELERATION: f32 = 0.6;
    const JUMP_VELOCITY: f32 = 2.5;
    const JUMP_TIME: i32 = 9;
    const JUMP_BUFFER: i32 = 2;
    const COYOTE_THRESHOLD: i32 = 5;
    const DASH_VELOCITY: f32 = 8.0;
    const DASH_ACCELERATION: f32 = 1.2;
    const DASH_TIME: i32 = 8;
    const DASH_WAIT_TIME: i32 = 32;
    const CONVEYOR_ACCELERATION: f32 = 0.2;
    const CONVEYOR_SPEED: f32 = 2.0;
    const MAX_FALL_HEIGHT: i32 = 30;
    const START_POSITION: Point = Point {
        x: WIDTH / 2 - TILE_WIDTH,
        y: HEIGHT - TILE_WIDTH - TILE_HEIGHT,
    };

    pub fn with_start_position(start_position: Point) -> Self {
        Blutti {
            position: start_position,
            start_position,
            ..Blutti::default()
        }
    }

    pub fn at_new_level(&self, start_position: Point, current_level: LevelNumber) -> Self {
        Blutti {
            position: start_position,
            start_position,
            points: self.points,
            lives: self.lives,
            current_level,
            ..Blutti::default()
        }
    }

    pub fn force_move(&mut self, velocity: Vec2) {
        self.remainder = self.remainder.add(velocity);
    }

    pub fn move_left(&mut self, speed: f32) {
        self.movement_modifier = speed;
        self.turn(DirectionX::Left);
        self.start_moving();
    }

    pub fn move_right(&mut self, speed: f32) {
        self.movement_modifier = speed;
        self.turn(DirectionX::Right);
        self.start_moving();
    }

    pub fn move_up(&mut self, speed: f32) {
        self.movement_modifier = speed;
        // Change direction
        if self.direction_y != DirectionY::Up {
            self.direction_y = DirectionY::Up;
            //self.add_climbing_animation();
        }
        self.start_climbing();
    }

    pub fn move_down(&mut self, speed: f32) {
        self.movement_modifier = speed;
        // Change direction
        if self.direction_y != DirectionY::Down {
            self.direction_y = DirectionY::Down;
            //self.add_climbing_animation();
        }
        self.start_climbing();
    }

    pub fn stop(&mut self) {
        if self.is_standing_on(TileCollider::Slippery) {
            return;
        }

        match self.state {
            PlayerState::RunningLeft | PlayerState::RunningRight => {
                self.stop_timer = Self::RUNNING_STOP_TIME;
                self.state = PlayerState::RunningStop
            }
            PlayerState::FallingLeft | PlayerState::FallingRight => {
                self.state = PlayerState::Falling;
            }
            PlayerState::ClimbingUp | PlayerState::ClimbingDown => {
                self.state = PlayerState::ClimbingStop
            }
            PlayerState::ClimbingSideways => self.state = PlayerState::ClimbingSidewaysStop,
            _ => (),
        }
    }

    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug
    }

    pub fn start_jump(&mut self) {
        if self.is_standing() || self.is_falling() && self.fall_timer < Self::COYOTE_THRESHOLD {
            self.jump();
        } else {
            //log_debug("jump bufferering");
            self.jump_buffer_timer = Self::JUMP_BUFFER;
        }
    }

    pub fn stop_jump(&mut self) {
        if self.is_jumping() {
            self.state = PlayerState::JumpingStop
        }
    }

    pub fn start_dash(&mut self) {
        if self.dash_timer < 0 {
            return;
        }
        if self.is_jumping() || self.is_falling() {
            //log_debug("start dashing");
            match self.state {
                PlayerState::JumpingLeft | PlayerState::FallingLeft => {
                    self.state = PlayerState::DashingLeft
                }
                PlayerState::JumpingRight | PlayerState::FallingRight => {
                    self.state = PlayerState::DashingRight
                }
                _ if self.direction_x == DirectionX::Left => self.state = PlayerState::DashingLeft,
                _ => self.state = PlayerState::DashingRight,
            }
            self.dash_timer = Self::DASH_TIME;
            self.add_dash_animation();
            play_sound("sound_dash");
        }
    }

    pub fn handle_effects(&mut self) {
        let state = get_state();
        if let Some(collision) = state
            .level
            .all_collisions_at_rect(self.position)
            .into_iter()
            .flatten()
            .last()
        {
            match collision.tile_collider {
                TileCollider::Collectable(points) => self.collect_collectable(collision, points),
                TileCollider::Deadly => {
                    //log_debug("death from deadly tile");
                    self.die();
                }
                TileCollider::Exit => self.exit(),
                TileCollider::ExtraLife => self.collect_extra_life(collision),
                TileCollider::Star => self.collect_star(collision),
                _ => (),
            }
        }
        state.blutti.current_tile = get_tile_index(self.position);
    }

    pub fn add_lives(&mut self, lives: i32) -> i32 {
        self.lives += lives;
        self.lives
    }

    pub fn add_points(&mut self, points: i32) -> i32 {
        self.points += points;
        self.points
    }

    pub fn die(&mut self) {
        if self.iddqd {
            return;
        }
        let state = get_state();
        state.level.reset();
        self.stop_movement(StopDirection::Both);
        self.state = PlayerState::Idle;
        self.add_death_animation();
        self.add_lives(-1);
        add_progress(get_me(), BADGE_DEATHS, 1);
        self.died = true;
        play_sound("sound_death");
    }

    pub fn reset(&mut self) {
        self.died = false;
        self.direction_x = DirectionX::Right;
        self.direction_y = DirectionY::Up;
        self.position = self.start_position;
        self.stop_movement(StopDirection::Both);
        self.add_idle_animation();
        self.current_tile = 0;
    }

    pub fn is_alive(&self) -> bool {
        self.lives > 0
    }

    fn start_moving(&mut self) {
        if self.is_on_ladder() {
            if self.state != PlayerState::ClimbingSideways {
                self.velocity.y = 0.0;
                self.state = PlayerState::ClimbingSideways;
            }
        } else if self.is_falling() {
            self.state = if self.direction_x == DirectionX::Right {
                PlayerState::FallingRight
            } else {
                PlayerState::FallingLeft
            };
        } else if self.is_jumping_up() {
            self.state = if self.direction_x == DirectionX::Right {
                PlayerState::JumpingRight
            } else {
                PlayerState::JumpingLeft
            };
        } else if self.is_standing() {
            if !self.is_running() {
                self.add_running_animation();
                self.state = if self.direction_x == DirectionX::Right {
                    PlayerState::RunningRight
                } else {
                    PlayerState::RunningLeft
                };
            }
        }
    }

    fn start_climbing(&mut self) {
        if self.can_climb()
            && !matches!(
                self.state,
                PlayerState::ClimbingUp | PlayerState::ClimbingDown
            )
        {
            self.velocity.x = 0.0;
            if !self.is_climbing() {
                self.add_climbing_animation();
            }
            //log_debug("start climbing");
            if self.direction_y == DirectionY::Up {
                self.state = PlayerState::ClimbingUp;
            } else {
                self.state = PlayerState::ClimbingDown;
            }
        }
    }

    fn start_idling(&mut self) {
        if self.is_on_ladder() {
            if self.state != PlayerState::ClimbingIdle {
                self.state = PlayerState::ClimbingIdle;
                self.add_climbing_animation();
            }
        } else if !self.is_standing() {
            self.state = PlayerState::Falling;
        } else if self.state != PlayerState::Idle {
            self.state = PlayerState::Idle;
            self.add_idle_animation();
        }
    }

    fn turn(&mut self, new_direction: DirectionX) {
        if self.direction_x != new_direction {
            self.direction_x = new_direction;
            self.add_turn_animation();
            self.add_running_animation();
        }
    }

    fn jump(&mut self) {
        play_sound("sound_jump");
        match self.state {
            PlayerState::RunningLeft => self.state = PlayerState::JumpingLeft,
            PlayerState::RunningRight => self.state = PlayerState::JumpingRight,
            _ => self.state = PlayerState::Jumping,
        };
        // Jumping when on slippery will decrease jump height
        self.jump_max_time = if self.is_standing_on(TileCollider::Slippery) {
            Self::JUMP_TIME / 2
        } else {
            Self::JUMP_TIME
        };
        self.jump_buffer_timer = 0;
        self.add_jump_animation();
    }

    fn is_climbing(&self) -> bool {
        match self.state {
            PlayerState::ClimbingUp
            | PlayerState::ClimbingDown
            | PlayerState::ClimbingStop
            | PlayerState::ClimbingIdle
            | PlayerState::ClimbingSideways
            | PlayerState::ClimbingSidewaysStop => true,
            _ => false,
        }
    }

    fn is_idling(&self) -> bool {
        match self.state {
            PlayerState::Idle | PlayerState::ClimbingIdle => true,
            _ => false,
        }
    }

    fn is_jumping(&self) -> bool {
        matches!(
            self.state,
            PlayerState::Jumping
                | PlayerState::JumpingLeft
                | PlayerState::JumpingRight
                | PlayerState::JumpingStop
        )
    }

    fn is_jumping_up(&self) -> bool {
        matches!(
            self.state,
            PlayerState::Jumping | PlayerState::JumpingLeft | PlayerState::JumpingRight
        )
    }

    fn is_running(&self) -> bool {
        matches!(
            self.state,
            PlayerState::RunningLeft | PlayerState::RunningRight | PlayerState::RunningStop
        )
    }

    fn is_dashing(&self) -> bool {
        matches!(
            self.state,
            PlayerState::DashingLeft | PlayerState::DashingRight
        )
    }

    fn collect_star(&mut self, collision: Collision) {
        self.handle_collection(collision);
        self.add_points(1);
        self.stars += 1;
        add_progress(get_me(), BADGE_STARS, 1);
        play_sound("sound_coin");
    }

    fn collect_collectable(&mut self, collision: Collision, points: i32) {
        self.handle_collection(collision);
        self.add_points(points);
        play_sound("sound_coin");
    }

    fn collect_extra_life(&mut self, collision: Collision) {
        self.handle_collection(collision);
        self.add_lives(1);
        play_sound("sound_powerup");
    }

    fn handle_collection(&mut self, collision: Collision) {
        let state = get_state();
        state.level.remove_tile(collision.position);
        let particle_position = get_origin_point_of_position(collision.position);
        self.add_collection_particle(particle_position);
    }

    fn exit(&mut self) {
        let state = get_state();
        if self.stars >= state.level.stars {
            self.finish_level();
        } else {
            let tile_pos = get_tile_index(self.position);
            if tile_pos != state.blutti.current_tile {
                play_sound("sound_wrong");
                state.blutti.current_tile = tile_pos;
            }
        }
    }

    fn finish_level(&mut self) {
        play_sound("sound_exit");
        self.finished_level = true;
        self.add_exit_animation();
        add_progress(get_me(), BADGE_LEVELS, 1);
    }

    fn can_climb(&self) -> bool {
        match self.direction_y {
            DirectionY::Up => self.is_on_ladder(),
            DirectionY::Down => self.is_on_ladder_below(),
        }
    }

    fn is_on_ladder(&self) -> bool {
        if self.direction_x == DirectionX::Left {
            if self.direction_y == DirectionY::Down {
                self.collision(self.position.below_bottom_left()) == TileCollider::Climbable
                    || self.collision(self.position.below_bottom_right().addx(-3))
                        == TileCollider::Climbable
            } else {
                self.collision(self.position.bottom_left()) == TileCollider::Climbable
                    || self.collision(self.position.bottom_right().addx(-3))
                        == TileCollider::Climbable
            }
        } else {
            if self.direction_y == DirectionY::Down {
                self.collision(self.position.below_bottom_left().addx(3)) == TileCollider::Climbable
                    || self.collision(self.position.below_bottom_right()) == TileCollider::Climbable
            } else {
                self.collision(self.position.bottom_left().addx(3)) == TileCollider::Climbable
                    || self.collision(self.position.bottom_right()) == TileCollider::Climbable
            }
        }
    }

    fn is_on_ladder_below(&self) -> bool {
        if self.direction_x == DirectionX::Left {
            self.collision(self.position.below_bottom_left()) == TileCollider::Climbable
                || self.collision(self.position.below_bottom_right().addx(-3))
                    == TileCollider::Climbable
        } else {
            self.collision(self.position.below_bottom_left().addx(3)) == TileCollider::Climbable
                || self.collision(self.position.below_bottom_right()) == TileCollider::Climbable
        }
    }

    fn add_idle_animation(&mut self) {
        self.animation = match self.direction_x {
            DirectionX::Left => Animation::animation_idle_left(),
            DirectionX::Right => Animation::animation_idle_right(),
        }
    }

    fn add_running_animation(&mut self) {
        self.animation = match self.direction_x {
            DirectionX::Left => Animation::animation_running_left(),
            DirectionX::Right => Animation::animation_running_right(),
        }
    }

    fn add_death_animation(&mut self) {
        self.animation = Animation::animation_death()
    }

    fn add_exit_animation(&mut self) {
        self.animation = match self.direction_x {
            DirectionX::Left => Animation::animation_exit_left(),
            DirectionX::Right => Animation::animation_exit_right(),
        }
    }

    fn add_climbing_animation(&mut self) {
        self.animation = match self.direction_x {
            DirectionX::Right => Animation::animation_climb_right(),
            DirectionX::Left => Animation::animation_climb_left(),
        }
    }

    fn add_jump_animation(&self) {
        match self.direction_x {
            DirectionX::Left => self.add_jump_particle(BLUTTI_JUMP_LEFT_SPRITES),
            DirectionX::Right => self.add_jump_particle(BLUTTI_JUMP_RIGHT_SPRITES),
        }
    }

    fn add_dash_animation(&self) {
        match self.direction_x {
            DirectionX::Left => self.add_dash_particle(BLUTTI_DASH_LEFT_SPRITES, TILE_WIDTH),
            DirectionX::Right => self.add_dash_particle(BLUTTI_DASH_RIGHT_SPRITES, -TILE_WIDTH),
        }
    }

    fn add_turn_animation(&self) {
        match self.direction_x {
            DirectionX::Left => self.add_turn_particle(BLUTTI_TURN_LEFT_SPRITES),
            DirectionX::Right => self.add_turn_particle(BLUTTI_TURN_RIGHT_SPRITES),
        }
    }

    fn add_dash_particle(&self, sprites: [i32; 4], offset_x: i32) {
        let anim = Animation::once(sprites.into(), 5);
        let position = Point {
            x: self.position.x + offset_x,
            y: self.position.y,
        };
        let particle = Particle::following(position, anim, offset_x);
        let state = get_state();
        state.level.particles.push(particle);
    }

    fn add_jump_particle(&self, sprites: [i32; 4]) {
        self.add_particle(self.position.bottom_left(), sprites.into());
    }

    fn add_turn_particle(&self, sprites: [i32; 4]) {
        let position = match self.direction_x {
            DirectionX::Left => self.position.bottom_left(),
            DirectionX::Right => self.position.bottom_right(),
        };
        self.add_particle(position.addy(-2), sprites.into());
    }

    fn add_collection_particle(&self, position: Point) {
        self.add_particle(position, COLLECTION_SPRITES.into());
    }

    fn add_particle(&self, position: Point, sprites: Vec<i32>) {
        let anim = Animation::once(sprites, 5);
        let particle = Particle::stationary(position, anim);
        let state = get_state();
        state.level.particles.push(particle);
    }

    fn update_horizontal_velocity(&mut self) {
        let moving_left = self.direction_x == DirectionX::Left;
        let (mut acceleration, mut target_velocity) = match self.state {
            PlayerState::RunningLeft => (-Self::RUNNING_ACCELERATION, -Self::MAX_VELOCITY),
            PlayerState::RunningRight => (Self::RUNNING_ACCELERATION, Self::MAX_VELOCITY),
            PlayerState::RunningStop if moving_left => (0.5, 0.0),
            PlayerState::RunningStop => (-0.5, 0.0),
            PlayerState::Jumping => (0.0, 0.0),
            PlayerState::JumpingLeft => (-Self::JUMP_ACCELERATION, -Self::JUMP_VELOCITY),
            PlayerState::JumpingRight => (Self::JUMP_ACCELERATION, Self::JUMP_VELOCITY),
            PlayerState::JumpingStop => {
                if self.velocity.x > 0.0 {
                    (Self::JUMP_ACCELERATION, Self::JUMP_VELOCITY)
                } else if self.velocity.x < 0.0 {
                    (-Self::JUMP_ACCELERATION, -Self::JUMP_VELOCITY)
                } else {
                    (0.0, 0.0)
                }
            }
            PlayerState::DashingLeft => (-Self::DASH_ACCELERATION, -Self::DASH_VELOCITY),
            PlayerState::DashingRight => (Self::DASH_ACCELERATION, Self::DASH_VELOCITY),
            PlayerState::ClimbingSideways if moving_left => (-0.3, -1.5),
            PlayerState::ClimbingSideways => (0.3, 1.5),
            PlayerState::ClimbingSidewaysStop if moving_left => (0.5, 0.0),
            PlayerState::ClimbingSidewaysStop => (-0.5, 0.0),
            PlayerState::ClimbingUp
            | PlayerState::ClimbingDown
            | PlayerState::ClimbingStop
            | PlayerState::Idle
            | PlayerState::ClimbingIdle
            | PlayerState::Falling => (0.0, 0.0),
            PlayerState::FallingLeft => {
                (-Self::FALLING_X_ACCELERATION, -Self::MAX_FALLING_VELOCITY)
            }
            PlayerState::FallingRight => (Self::FALLING_X_ACCELERATION, Self::MAX_FALLING_VELOCITY),
        };
        acceleration *= self.movement_modifier;
        target_velocity *= self.movement_modifier;

        // Handle conveyor
        if self.is_standing_on(TileCollider::Conveyor) {
            acceleration += Self::CONVEYOR_ACCELERATION;
            if target_velocity > 0.0 {
                target_velocity += Self::CONVEYOR_SPEED;
            } else {
                target_velocity -= Self::CONVEYOR_SPEED;
            }
        }

        if target_velocity > 0.0 {
            self.velocity.x = (self.velocity.x + acceleration).min(target_velocity);
        } else if target_velocity < 0.0 {
            self.velocity.x = (self.velocity.x + acceleration).max(target_velocity);
        } else if acceleration > 0.0 {
            self.velocity.x = (self.velocity.x + acceleration).min(target_velocity);
        } else if acceleration < 0.0 {
            self.velocity.x = (self.velocity.x + acceleration).max(target_velocity);
        } else {
            self.velocity.x = 0.0;
        }
    }

    fn update_vertical_velocity(&mut self) {
        let (mut acceleration, mut target_velocity) = match self.state {
            PlayerState::Jumping | PlayerState::JumpingLeft | PlayerState::JumpingRight => {
                (-1.5, -2.0)
            }
            PlayerState::JumpingStop => (2.0, 5.0),
            PlayerState::ClimbingUp => (-0.4, -1.0),
            PlayerState::ClimbingDown => (0.4, 1.0),
            PlayerState::ClimbingStop => (-0.2, 0.0),
            PlayerState::ClimbingIdle
            | PlayerState::ClimbingSideways
            | PlayerState::ClimbingSidewaysStop
            | PlayerState::DashingLeft
            | PlayerState::DashingRight
            | PlayerState::RunningLeft
            | PlayerState::RunningRight
            | PlayerState::RunningStop
            | PlayerState::Idle => (0.0, 0.0),
            PlayerState::Falling | PlayerState::FallingLeft | PlayerState::FallingRight => {
                (GRAVITY_ACCELERATION, GRAVITY_MAX)
            } // Gravity
        };
        acceleration *= self.movement_modifier;
        target_velocity *= self.movement_modifier;

        if target_velocity > 0.0 {
            self.velocity.y = (self.velocity.y + acceleration).min(target_velocity);
        } else if target_velocity < 0.0 {
            self.velocity.y = (self.velocity.y + acceleration).max(target_velocity);
        } else if acceleration > 0.0 {
            self.velocity.y = (self.velocity.y + acceleration).min(target_velocity);
        } else if acceleration < 0.0 {
            self.velocity.y = (self.velocity.y + acceleration).max(target_velocity);
        } else {
            self.velocity.y = 0.0;
        }
    }

    fn update_states(&mut self) {
        if self.is_falling() {
            self.fall_timer += 1;
        }

        // Jump buffering to allow jump being pressed slightly too early
        if self.is_standing() && self.jump_buffer_timer > 0 {
            //log_debug("buffer jump!");
            self.jump();
        }
        if self.is_falling() || self.is_jumping() {
            self.jump_buffer_timer -= 1;
        }

        // Climbing
        let on_ladder = self.is_on_ladder();
        if !self.is_moving() && !self.is_idling() && !self.is_jumping()
            || self.is_climbing() && !on_ladder
            || self.state == PlayerState::Idle && on_ladder
        {
            //log_debug(str_format!(str32, "stop movement from {:?}", self.state).as_str());
            //log_debug(str_format!(str32, "velocity {:?}", self.velocity).as_str());
            self.stop_movement(StopDirection::Both);
        }

        // Stop running
        if self.state == PlayerState::RunningStop {
            if self.stop_timer > 0 {
                self.stop_timer -= 1;
            } else {
                self.stop_movement(StopDirection::X);
            }
        }

        // Jumping
        if self.is_jumping() {
            if self.state == PlayerState::JumpingStop {
                self.jump_timer -= 1;
                if self.jump_timer <= 0 {
                    //log_debug(str_format!(str32, "jump > falling timer:{}", self.jump_timer).as_str());
                    self.stop_movement(StopDirection::Y);
                }
            } else {
                self.jump_timer += 1;
                if self.jump_timer > self.jump_max_time {
                    self.state = PlayerState::JumpingStop;
                }
            }
        }

        // Dashing
        if self.is_dashing() {
            self.dash_timer -= 1;
            //log_debug(str_format!(str32, "dash_timer: {}", self.dash_timer).as_str());
            if self.dash_timer == 0 {
                //log_debug("stop dashing");
                self.stop_movement(StopDirection::X);
                self.dash_timer = -Self::DASH_WAIT_TIME;
            }
        } else if self.dash_timer < 0 {
            //log_debug(str_format!(str32, "increasing dash_timer: {}", self.dash_timer).as_str());
            self.dash_timer += 1;
        }

        // Death from fall height
        if self.is_standing() {
            if self.fall_timer > Self::MAX_FALL_HEIGHT {
                //log_debug(str_format!(str32, "die from fall height {}", self.fall_timer).as_str());
                self.die();
            }
        } else if !self.is_jumping() && !self.is_dashing() {
            //log_debug(str_format!(str32, "state {:?} > Falling", self.state).as_str());
            self.stop_movement(StopDirection::Y);
        }

        // Death from falling out of screen
        if self.position.y >= (HEIGHT - TILE_HEIGHT) {
            //log_debug("die from falling out of screen");
            self.die();
        }

        // Death from deadly monsters
        let state = get_state();
        if state.level.deadly_monsters_overlapping_rect(self.rect()) {
            //log_debug("die from monster");
            state.blutti.die();
        }
    }

    fn is_moving(&self) -> bool {
        !self.velocity.is_zero() && !self.remainder.is_zero()
    }

    fn is_falling(&self) -> bool {
        matches!(
            self.state,
            PlayerState::Falling | PlayerState::FallingLeft | PlayerState::FallingRight
        )
    }
}

impl Default for Blutti {
    fn default() -> Self {
        Self {
            position: Self::START_POSITION,
            start_position: Self::START_POSITION,
            jump_timer: 0,
            dash_timer: 0,
            fall_timer: 0,
            stop_timer: 0,
            jump_max_time: 0,
            jump_buffer_timer: 0,
            direction_x: DirectionX::Right,
            direction_y: DirectionY::Up,
            state: PlayerState::Idle,
            velocity: Vec2::zero(),
            remainder: Vec2::zero(),
            movement_modifier: 1.0,
            points: 0,
            stars: 0,
            lives: 3,
            iddqd: false,
            died: false,
            finished_level: false,
            current_level: 1,
            current_tile: 0,
            animation: Animation::animation_idle_right(),
            debug: false,
        }
    }
}

impl Drawable for Blutti {
    fn draw(&self) {
        if !self.animation.finished {
            let tile = self.animation.current_sprite();
            draw_tile(tile, self.position());
        }

        // Player debug
        if self.debug {
            self.draw_debug();
        }
    }

    fn draw_debug(&self) {
        let mut textpos = self.position().clone();
        textpos.x = textpos.x.min(192);
        textpos.y -= 4;
        display_text(str_format!(str32, "{:?}", self.state).as_str(), textpos);
        textpos.y -= 8;
        display_text(
            str_format!(str32, "Y {}", self.position().y).as_str(),
            textpos,
        );
        textpos.y -= 8;
        display_text(
            str_format!(str32, "X {}", self.position().x).as_str(),
            textpos,
        );
        textpos.y -= 8;
        display_text(
            str_format!(str32, "VY {:.2}", self.velocity.x).as_str(),
            textpos,
        );
        textpos.y -= 8;
        display_text(
            str_format!(str32, "VY {:.2}", self.velocity.y).as_str(),
            textpos,
        );
        textpos.y -= 8;
        display_text(
            str_format!(str32, "JT {:}", self.jump_timer).as_str(),
            textpos,
        );
        textpos.y -= 8;
        display_text(
            str_format!(str32, "JB {:}", self.jump_buffer_timer).as_str(),
            textpos,
        );
        textpos.y -= 8;
        display_text(
            str_format!(str32, "FT {:}", self.fall_timer).as_str(),
            textpos,
        );
        textpos.y -= 8;
        display_text(
            str_format!(str32, "DT {:}", self.dash_timer).as_str(),
            textpos,
        );
        textpos.y -= 8;
        display_text(
            str_format!(str32, "OnLadder {}", self.is_on_ladder()).as_str(),
            textpos,
        );
        textpos.y -= 8;
        display_text(
            str_format!(str32, "CanClimb {}", self.can_climb()).as_str(),
            textpos,
        );
        textpos.y -= 8;
        display_text(
            str_format!(str32, "Climbing {}", self.is_climbing()).as_str(),
            textpos,
        );
    }
}

impl Updateable for Blutti {
    fn position(&self) -> Point {
        self.position
    }

    fn update(&mut self) {
        self.animation.update();

        // Horizontal movement
        self.update_horizontal_velocity();

        // Vertical movement
        self.update_vertical_velocity();

        // Move X position
        (self.position, self.remainder) =
            self.move_horizontally(self.position, self.velocity, self.remainder);

        // Move y position
        (self.position, self.remainder) =
            self.move_vertically(self.position, self.velocity, self.remainder);

        // Update states
        self.update_states();
    }

    fn stop_movement(&mut self, stop_direction: StopDirection) {
        self.jump_timer = 0;
        self.dash_timer = 0;
        if stop_direction != StopDirection::X {
            self.fall_timer = 0;
        }
        self.movement_modifier = 1.0;
        self.remainder = Vec2::zero();
        match stop_direction {
            StopDirection::X => self.velocity.x = 0.0,
            StopDirection::Y => self.velocity.y = 0.0,
            StopDirection::Both => self.velocity = Vec2::zero(),
        };
        self.start_idling();
    }

    fn collision_at(&self, position: Point) -> bool {
        if self.is_climbing() {
            if self.direction_x == DirectionX::Left {
                !(self.is_position_free(position.addx(3))
                    && self.is_position_free(position.top_right().addx(-4))
                    && self.is_position_free(position.bottom_left().addx(3))
                    && self.is_position_free(position.bottom_right().addx(-4)))
            } else {
                !(self.is_position_free(position.addx(4))
                    && self.is_position_free(position.top_right().addx(-3))
                    && self.is_position_free(position.bottom_left().addx(4))
                    && self.is_position_free(position.bottom_right().addx(-3)))
            }
        } else {
            if self.direction_x == DirectionX::Left {
                !(self.is_position_free(position)
                    && self.is_position_free(position.top_right().addx(-3))
                    && self.is_position_free(position.bottom_left())
                    && self.is_position_free(position.bottom_right().addx(-3)))
            } else {
                !(self.is_position_free(position.addx(3))
                    && self.is_position_free(position.top_right())
                    && self.is_position_free(position.bottom_left().addx(3))
                    && self.is_position_free(position.bottom_right()))
            }
        }
    }

    fn is_standing(&self) -> bool {
        if self.is_standing_on(TileCollider::Climbable) {
            return true;
        }

        !(self.is_position_free(self.position().below_bottom_left())
            && self.is_position_free(self.position().below_bottom_right()))
    }

    fn is_standing_on(&self, collision: TileCollider) -> bool {
        self.collision(self.position().below_bottom_left()) == collision
            || self.collision(self.position().below_bottom_right()) == collision
    }
}
