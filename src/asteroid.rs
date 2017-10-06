use piston_window::{polygon, math, Transformed, Graphics, ImageSize};
use cgmath::{Point2, Vector2, Rad};
use rustfest_game_assets::ASTEROIDS;
use rand::random;

use std::usize;

const RED: [f32; 4] = [1., 0., 0., 1.];
const ASTEROID_SCALE: f64 = 0.08;
const ASTEROID_MAX_SPEED: f64 = 0.4;
const ASTEROID_MAX_SPIN: f64 = 0.5;
const TAU: f64 = 6.283185;

// TODO:
// - Implement collisions - on collision change level, sprite, velocity, rotation, angular velocity
// - Change scale with level

pub struct Asteroid {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    rotation: Rad<f64>,
    spin: Rad<f64>,
    level: u8,
    sprite: usize,
}

fn rand(min: f64, range: f64) -> f64 {
    random::<f64>() * range + min
}

impl Asteroid {
    pub fn new() -> Asteroid {
        let velocity = Vector2 {
            x: rand(-ASTEROID_MAX_SPEED, 2. * ASTEROID_MAX_SPEED),
            y: rand(-ASTEROID_MAX_SPEED, 2. * ASTEROID_MAX_SPEED),
        };
        Asteroid {
            position: Point2 { x: rand(-1., 2.), y: rand(-1., 2.) },
            velocity,
            rotation: Rad(rand(0., TAU)),
            spin: Rad(rand(0., ASTEROID_MAX_SPIN)),
            level: 3,
            sprite: (random::<f32>() * 5.) as usize,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.position += self.velocity * dt;
        self.rotation += self.spin * dt;

        // Wrap position to the screen
        if self.position.x.abs() > 1. { self.position.x *= -1.; }
        if self.position.y.abs() > 1. { self.position.y *= -1.; }
    }

    pub fn render<G, T>(&self, graphics: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize
    {
        polygon(
            RED,
            ASTEROIDS[self.sprite],
            math::identity()
                .trans(self.position.x, self.position.y)
                .scale(ASTEROID_SCALE, ASTEROID_SCALE)
                .rot_rad(self.rotation.0),
            graphics,
        )
    }
}
