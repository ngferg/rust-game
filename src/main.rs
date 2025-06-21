use std::f32::consts::PI;
use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;

const TURNING_FACTOR: f32 = 8.0;
const ACCELERATION_FACTOR: f32 = 0.25;
const MAX_SPEED: f32 = 2.0;
const PHYSICS_TICK_RATE: f64 = 144.0;
const ANIMATED_INDEX: usize = 1;
const IDLE_INDEX: usize = 0;

const MAX_X: f32 = 1280.0;
const MAX_Y: f32 = 720.0;

struct Player {
    x: f32,
    y: f32,
    angle: f32,
    speed: f32,
}

impl Player {
    fn get_animation_style(&self) -> usize {
        match self.speed {
            0.0 => IDLE_INDEX,
            _ => ANIMATED_INDEX,
        }
    }

    fn accelerate(&mut self) {
        self.speed += ACCELERATION_FACTOR;
        if self.speed > MAX_SPEED {
            self.speed = MAX_SPEED;
        }
    }

    fn decelerate(&mut self) {
        self.speed -= ACCELERATION_FACTOR;
        if self.speed < 0.0 {
            self.speed = 0.0;
        }
    }

    fn turn_left(&mut self) {
        self.angle -= PI / TURNING_FACTOR;
        if self.angle < 0.0 {
            self.angle += 2.0 * PI;
        }
    }

    fn turn_right(&mut self) {
        self.angle += PI / TURNING_FACTOR;
        if self.angle > 2.0 * PI {
            self.angle -= 2.0 * PI
        }
    }

    fn process_movement(&mut self) {
        self.y -= self.speed * self.angle.cos();
        self.x += self.speed * self.angle.sin();
        if self.x > MAX_X {
            self.x -= MAX_X;
        } else if self.x < -32.0 {
            self.x += MAX_X + 32.0;
        }
        if self.y > MAX_Y {
            self.y -= MAX_Y;
        } else if self.y < -32.0 {
            self.y += MAX_Y + 32.0;
        }
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut last_physics_tick = now();
    request_new_screen_size(MAX_X, MAX_Y);

    let ship_png: &Texture2D = &load_texture("assets/ship_moving.png")
        .await
        .expect("Ship moving image failed to load!");
    let mut player = Player {
        x: MAX_X / 2.0,
        y: MAX_Y / 2.0,
        angle: 0.0,
        speed: 0.0,
    };
    let mut ship_sprite = AnimatedSprite::new(
        32,
        32,
        &[
            Animation {
                name: "idle".to_string(),
                row: 1,
                frames: 1,
                fps: 1,
            },
            Animation {
                name: "run".to_string(),
                row: 0,
                frames: 9,
                fps: 60,
            },
        ],
        true,
    );

    loop {
        clear_background(BLACK);

        match get_last_key_pressed() {
            Some(KeyCode::Escape) => break,
            Some(KeyCode::W) => player.accelerate(),
            Some(KeyCode::S) => player.decelerate(),
            Some(KeyCode::A) => player.turn_left(),
            Some(KeyCode::D) => player.turn_right(),
            _ => {}
        }

        let now = now();
        if now > last_physics_tick + (1.0/PHYSICS_TICK_RATE) {
            last_physics_tick = miniquad::date::now();
            player.process_movement();
        }

        ship_sprite.set_animation(player.get_animation_style());
        draw_texture_ex(
            ship_png,
            player.x,
            player.y,
            WHITE,
            DrawTextureParams {
                source: Some(ship_sprite.frame().source_rect),
                dest_size: Some(ship_sprite.frame().dest_size),
                rotation: player.angle,
                ..Default::default()
            },
        );
        // Update frame
        ship_sprite.update();
        next_frame().await;
    }
}
