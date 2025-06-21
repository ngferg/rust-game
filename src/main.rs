use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use std::f32::consts::PI;
use ::rand::Rng;

const FONT_SIZE: f32 = 32.0;
const TURNING_FACTOR: f32 = 8.0;
const ACCELERATION_FACTOR: f32 = 0.25;
const MAX_SPEED: f32 = 2.0;
const PHYSICS_TICK_RATE: f64 = 144.0;
const ANIMATED_INDEX: usize = 0;
const IDLE_INDEX: usize = 1;
const BOOM_INDEX: usize = 2;
const MAX_X: f32 = 1280.0;
const MAX_Y: f32 = 720.0;
const BULLET_SPEED: f32 = 10.0;
const MAX_BULLETS: usize = 3;
const ASTROID_MAX_SIZE: u8 = 5;
const ASTROID_ANGLE_RANGE: f32 = 0.2;
const ASTROID_BUFFER_ZONE: f32 = 50.0;
const STARTING_SPAWN_RATE: u64 = PHYSICS_TICK_RATE as u64 * 5;
const ASTROID_ACCELERATION_FACTOR: f64 = 10.0;
const ASTROID_SPEED: f32 = 1.0;
const ASTROID_RADIUS_FACTOR: f32 = 5.0;
const ACCURACY_LEEWAY: f32 = 3.0;
const SHIP_HIT_LEEWAY: f32 = 4.0;

struct Player {
    x: f32,
    y: f32,
    angle: f32,
    speed: f32,
    bullets: Vec<Bullet>,
}

impl Player {

    fn new(x: f32, y: f32) -> Self {
        Player {
            x, y, angle: 0.0, speed: 0.0, bullets: vec![]
        }
    }

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

    fn shoot(&mut self) {
        if self.bullets.len() < MAX_BULLETS {
            let color = if self.bullets.len() == MAX_BULLETS - 1 {
                RED
            } else {
                WHITE
            };
            self.bullets.push(Bullet::new(self.x + 16.0, self.y + 16.0, self.angle, color));
        }
    }
    
    fn is_hit(&self, astroids: &Vec<Astroid>) -> bool {
        astroids.iter()
            .any(|astroid| self.x + 16.0 > astroid.x - astroid.size as f32 * ASTROID_RADIUS_FACTOR - SHIP_HIT_LEEWAY &&
                self.x + 16.0 < astroid.x + astroid.size as f32 * ASTROID_RADIUS_FACTOR + SHIP_HIT_LEEWAY &&
                self.y + 16.0 > astroid.y - astroid.size as f32 * ASTROID_RADIUS_FACTOR - SHIP_HIT_LEEWAY &&
                self.y + 16.0 < astroid.y + astroid.size as f32 * ASTROID_RADIUS_FACTOR + SHIP_HIT_LEEWAY)
    } 
}

fn is_on_screen(x: f32, y: f32) -> bool{
    !(x < -10.0 || x > MAX_X + 10.0 || y < -10.0 || y > MAX_Y + 10.0)
}

struct Bullet {
    x: f32,
    y: f32,
    angle: f32,
    color: Color,
}

impl Bullet {

    fn new(x: f32, y: f32, angle: f32, color: Color) -> Self {
        Bullet {
            x, y, angle, color
        }
    }

    fn process_movement(&mut self) {
        self.y -= BULLET_SPEED * self.angle.cos();
        self.x += BULLET_SPEED * self.angle.sin();
    }

    fn is_on_screen(&self) -> bool {
        is_on_screen(self.x, self.y)
    }

    fn intersects_astroid_at_index(&self, astroids: &Vec<Astroid>) -> Option<usize> {
        astroids.iter()
            .position(|astroid| self.x > astroid.x - astroid.size as f32 * ASTROID_RADIUS_FACTOR - ACCURACY_LEEWAY &&
                self.x < astroid.x + astroid.size as f32 * ASTROID_RADIUS_FACTOR + ACCURACY_LEEWAY &&
                self.y > astroid.y - astroid.size as f32 * ASTROID_RADIUS_FACTOR - ACCURACY_LEEWAY &&
                self.y < astroid.y + astroid.size as f32 * ASTROID_RADIUS_FACTOR + ACCURACY_LEEWAY)
    }
}

struct Astroid {
    x: f32,
    y: f32,
    angle: f32,
    size: u8,
}

impl Astroid {
    fn new() -> Self {
        let mut rng = ::rand::rng();
        match rng.random_range(0..4) {
            0 => Astroid { x: 0.0, y: rng.random_range(ASTROID_BUFFER_ZONE..MAX_Y - ASTROID_BUFFER_ZONE), angle: rng.random_range(ASTROID_ANGLE_RANGE..PI-ASTROID_ANGLE_RANGE), size: rng.random_range(1..=ASTROID_MAX_SIZE)},
            1 => Astroid { x: MAX_X, y: rng.random_range(ASTROID_BUFFER_ZONE..MAX_Y - ASTROID_BUFFER_ZONE), angle: rng.random_range(PI + ASTROID_ANGLE_RANGE..2.0*PI-ASTROID_ANGLE_RANGE), size: rng.random_range(1..=ASTROID_MAX_SIZE)},
            2 => Astroid { x: rng.random_range(ASTROID_BUFFER_ZONE..MAX_X - ASTROID_BUFFER_ZONE), y: 0.0, angle: rng.random_range(PI/2.0 + ASTROID_ANGLE_RANGE..3.0*PI/2.0-ASTROID_ANGLE_RANGE), size: rng.random_range(1..=ASTROID_MAX_SIZE)},
            _ => Astroid { x: rng.random_range(ASTROID_BUFFER_ZONE..MAX_X - ASTROID_BUFFER_ZONE), y: MAX_Y, angle: rng.random_range(-PI/2.0 + ASTROID_ANGLE_RANGE..PI/2.0-ASTROID_ANGLE_RANGE), size: rng.random_range(1..=ASTROID_MAX_SIZE)},
        }
    }

    fn process_movement(&mut self) {
        self.y -= ASTROID_SPEED * self.angle.cos();
        self.x += ASTROID_SPEED * self.angle.sin();
    }

    fn is_on_screen(&self) -> bool {
        is_on_screen(self.x, self.y)
    }
}

struct Astroids {
    astroids: Vec<Astroid>,
    spawn_rate: u64,
    spawn_counter: u64,
}

impl Astroids {
    fn new() -> Self {
        Astroids {
            astroids: vec![],
            spawn_rate: STARTING_SPAWN_RATE,
            spawn_counter: 0,
        }
    }
}


#[macroquad::main("MyGame")]
async fn main() {
    let mut booming = false;
    let mut score: u64 = 0;
    let mut last_second = now() as u64;
    let mut frames: u64 = 0;
    let mut last_frames: u64 = 0;
    let mut last_physics_tick = now();
    request_new_screen_size(MAX_X, MAX_Y);

    let ship_png: &Texture2D = &load_texture("assets/ship.png")
        .await
        .expect("Ship image failed to load!");
    let mut player = Player::new(MAX_X / 2.0, MAX_Y / 2.0);
    let mut ship_sprite = AnimatedSprite::new(
        32,
        32,
        &[
            Animation {
                name: "moving".to_string(),
                row: ANIMATED_INDEX as u32,
                frames: 9,
                fps: 48,
            },
            Animation {
                name: "idle".to_string(),
                row: IDLE_INDEX as u32,
                frames: 1,
                fps: 1,
            },
            Animation {
                name: "boom".to_string(),
                row: BOOM_INDEX as u32,
                frames: 20,
                fps: 48,
            }
        ],
        true,
    );

    let mut astroids = Astroids::new();

    loop {
        frames += 1;
        if now() as u64 > last_second {
            last_frames = frames;
            frames = 1;
            last_second = now() as u64;
        }
        clear_background(BLACK);


        match get_last_key_pressed() {
            Some(KeyCode::Escape) => {
                booming = true;
            },
            Some(KeyCode::W) => player.accelerate(),
            Some(KeyCode::S) => player.decelerate(),
            Some(KeyCode::A) => player.turn_left(),
            Some(KeyCode::D) => player.turn_right(),
            Some(KeyCode::B) => astroids.astroids.push(Astroid::new()),
            Some(KeyCode::Space) => player.shoot(),
            _ => {}
        }

        if now() > last_physics_tick + (1.0 / PHYSICS_TICK_RATE) {
            last_physics_tick = now();
            player.process_movement();
            player.bullets.iter_mut()
                .for_each(|bullet| bullet.process_movement());


            player.bullets = player.bullets.into_iter()
                .filter(|bullet| bullet.is_on_screen())
                .filter(|bullet| {
                    let astroid_hit = bullet.intersects_astroid_at_index(&astroids.astroids);
                    match astroid_hit {
                        Some(i) => {
                            astroids.astroids[i].size -= 1;
                            if astroids.astroids[i].size == 0 {
                                astroids.astroids.remove(i);
                                score += 1;
                            }
                            false
                        },
                        _ => true
                    }
                })
                .collect();

            astroids.astroids.iter_mut()
                .for_each(|astroid| astroid.process_movement());
            
            booming = booming || player.is_hit(&astroids.astroids);

            astroids.spawn_counter += 1;
            if astroids.spawn_counter > astroids.spawn_rate {
                astroids.astroids.push(Astroid::new());
                astroids.spawn_counter = 0;
                astroids.spawn_rate -= (PHYSICS_TICK_RATE / ASTROID_ACCELERATION_FACTOR) as u64;
                if astroids.spawn_rate < PHYSICS_TICK_RATE as u64 {
                    astroids.spawn_rate = PHYSICS_TICK_RATE as u64;
                }
            }

            astroids.astroids = astroids.astroids.into_iter()
                .filter(|astroid| astroid.is_on_screen())
                .collect();
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
        player.bullets.iter()
            .for_each(|bullet| draw_circle(bullet.x, bullet.y, 1.0, bullet.color));
        astroids.astroids.iter()
            .for_each(|astroid| draw_circle(astroid.x, astroid.y, astroid.size as f32 * ASTROID_RADIUS_FACTOR, LIGHTGRAY));
        ship_sprite.update();
        draw_text(format!("Score {}, fps: {}", score, last_frames).as_str(), 0.0, MAX_Y - FONT_SIZE, FONT_SIZE, LIGHTGRAY);
        next_frame().await;

        if booming {
            while booming {
                ship_sprite.set_animation(BOOM_INDEX);
                clear_background(BLACK);
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
                ship_sprite.update();
                booming = !ship_sprite.is_last_frame();
                next_frame().await;
            }
            break;
        }
    }
}
