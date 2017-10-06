use piston_window::{polygon, math, Transformed, Graphics, ImageSize};
use cgmath::{Point2, Vector2, Rad};
use rustfest_game_assets::ASTEROIDS;
use rand::random;

use std::usize;

const RED: [f32; 4] = [1., 0., 0., 1.];
const ASTEROID_SCALE: f64 = 0.1;
const ASTEROID_SPIN: f64 = 0.3;

// TODO:
// - Randomise initial position, velocity, rotation, angular velocity, and sprite
// - Implement collisions - on collision change level, sprite, velocity, rotation, angular velocity
// - Change scale with level

pub struct Asteroid {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    rotation: Rad<f64>,
    level: u8,
    sprite: usize,
}

impl Asteroid {
    pub fn new() -> Asteroid {
        Asteroid {
            position: Point2 { x: random(), y: random() },
            velocity: random(),
            rotation: Rad(0.),
            level: 3,
            sprite: random::<usize>() / usize::MAX * 5,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.position += self.velocity * dt;
        self.rotation += Rad(ASTEROID_SPIN * dt);

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
