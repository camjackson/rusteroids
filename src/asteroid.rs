use piston_window::{Graphics, ImageSize};
use cgmath::{Point2, Vector2, Rad};
use rustfest_game_assets::ASTEROIDS;
use rand::random;

use transform::Transform;
use polygon::Polygon;

const RED: [f32; 4] = [1., 0., 0., 1.];
const ASTEROID_SCALES: [f64; 3] = [0.02, 0.05, 0.08];
const INITIAL_LEVEL: usize = 2;
const ASTEROID_MAX_SPEED: f64 = 0.4;
const ASTEROID_MAX_SPIN: f64 = 0.5;
const TAU: f64 = 6.283185;

// TODO:
// - Implement collisions - on collision change level, sprite, velocity, rotation, angular velocity

pub struct Asteroid {
    transform: Transform,
    polygon: Polygon,
    velocity: Vector2<f64>,
    spin: Rad<f64>,
    level: usize,
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
            transform: Transform {
                position: Point2 { x: rand(-1., 2.), y: rand(-1., 2.) },
                rotation: Rad(rand(0., TAU)),
                scale: Vector2 { x: ASTEROID_SCALES[INITIAL_LEVEL], y: ASTEROID_SCALES[INITIAL_LEVEL] }
            },
            polygon: Polygon {
                color: RED,
                polygon: ASTEROIDS[(random::<f32>() * 5.) as usize],
            },
            velocity,
            spin: Rad(rand(0., ASTEROID_MAX_SPIN)),
            level: INITIAL_LEVEL,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.transform.position += self.velocity * dt;
        self.transform.rotation += self.spin * dt;

        // Wrap position to the screen
        if self.transform.position.x.abs() > 1. { self.transform.position.x *= -1.; }
        if self.transform.position.y.abs() > 1. { self.transform.position.y *= -1.; }
    }

    pub fn render<G, T>(&self, graphics: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize
    {
        self.polygon.render(&self.transform, graphics);
    }
}
