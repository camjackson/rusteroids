use std::fs::File;
use std::io::BufReader;
use piston_window::Graphics;
use cgmath::{Basis2, Rotation2, Rotation, InnerSpace, Point2, Vector2, Rad};
use rustfest_game_assets::PLAYER;
use rodio;
use rodio::Source;

use controller::Controller;
use transform::Transform;
use polygon::Polygon;
use bullets::Bullets;
use asset_path;

const GREEN: [f32; 4] = [0., 1., 0., 1.];
const PLAYER_SCALE: f64 = 0.035;
const MAX_SPEED: f64 = 1.;
const ROTATION_SPEED: f64 = 2.8;
const THRUST: f64 = 0.7;
const FIRE_INTERVAL: f64 = 0.2;
const LIVES: u8 = 3;
const INITIAL_POSITION: Point2<f64> = Point2 { x: 0., y: 0.  };
const INITIAL_VELOCITY: Vector2<f64> = Vector2 { x: 0., y: 0.  };

pub struct Player {
    pub transform: Transform,
    polygon: Polygon,
    velocity: Vector2<f64>,
    time_since_fired: f64,
    lives: u8,
    pub score: u16,
}

impl Player {
    pub fn new() -> Player {
        Player {
            transform: Transform {
                position: INITIAL_POSITION,
                rotation: Rad(0.),
                scale: Vector2 { x: PLAYER_SCALE, y: PLAYER_SCALE },
            },
            polygon: Polygon {
                color: GREEN,
                polygon: PLAYER,
            },
            velocity: INITIAL_VELOCITY,
            time_since_fired: 0.,
            lives: LIVES,
            score: 0,
        }
    }

    pub fn update(&mut self, audio_endpoint: &rodio::Endpoint, controller: &Controller, bullets: &mut Bullets, dt: f64) {
        let acceleration = Basis2::from_angle(self.transform.rotation).rotate_vector(Vector2::unit_y())* THRUST;

        // Apply acceleration to the velocity
        if controller.up { self.velocity += acceleration * dt}
        if controller.down { self.velocity -= acceleration * dt; }
        if controller.left { self.transform.rotation += Rad(ROTATION_SPEED * dt); }
        if controller.right { self.transform.rotation -= Rad(ROTATION_SPEED * dt); }

        // Clamp velocity
        if self.velocity.magnitude() > MAX_SPEED {
            self.velocity = self.velocity.normalize_to(MAX_SPEED);
        }

        // Apply velocity to position
        self.transform.position += self.velocity * dt;

        // Wrap position to the screen
        if self.transform.position.x.abs() > 1. { self.transform.position.x *= -1.; }
        if self.transform.position.y.abs() > 1. { self.transform.position.y *= -1.; }

        // Count time since last fire
        self.time_since_fired += dt;

        // Fire
        if controller.fire && self.time_since_fired > FIRE_INTERVAL {
            self.time_since_fired = 0.;
            bullets.spawn(self.transform.position, self.transform.rotation);
            self.laser_sound(audio_endpoint)
        }
    }

    pub fn score(&mut self) {
        self.score += 1;
    }

    pub fn kill(&mut self) {
        if self.lives > 0 {
            self.transform.position = INITIAL_POSITION;
            self.lives -= 1;
        } else {
            self.transform.position = Point2 { x: 999., y: 999. }
        }
        self.velocity = INITIAL_VELOCITY;
    }

    pub fn render<G>(&self, graphics: &mut G)
        where G: Graphics
    {
        self.polygon.render(&self.transform, graphics);
        for i in 0..self.lives {
            let transform = Transform {
                position: Point2 { x: -0.9 + i as f64 * 0.1, y: -0.9 },
                rotation: Rad(0.),
                scale: Vector2 { x: PLAYER_SCALE * 0.66, y: PLAYER_SCALE * 0.66 },
            };
            self.polygon.render(&transform, graphics);
        }
    }

    fn laser_sound(&self, audio_endpoint: &rodio::Endpoint) {
        let buffer = BufReader::new(File::open(asset_path("laser.wav")).unwrap());
        let laser = rodio::Decoder::new(buffer).unwrap();
        rodio::play_raw(&audio_endpoint, laser.convert_samples());
    }
}
