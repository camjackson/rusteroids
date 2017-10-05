use piston_window::{polygon, math, Transformed, Graphics, ImageSize};
use cgmath::{Point2, Vector2, Basis2, Rotation2, Rotation, Rad};
use rustfest_game_assets::BULLET;

use controller::Controller;

const BULLET_LIFETIME: f64 = 0.5;
const RED: [f32; 4] = [1., 0., 0., 1.];
const BULLET_SCALE: f64 = 0.02;
const BULLET_SPEED: f64 = 4.;

pub struct Bullet {
    pub position: Point2<f64>,
    pub alive: bool,
    velocity: Vector2<f64>,
    age: f64,
}

impl Bullet {
    pub fn new(position: Point2<f64>, rotation: Rad<f64>) -> Bullet {
        let direction = Basis2::from_angle(rotation).rotate_vector(Vector2::unit_y());
        Bullet {
            position: position,
            velocity: direction * BULLET_SPEED,
            age: 0.,
            alive: true,
        }
    }

    pub fn update(&mut self, _: &Controller, dt: f64) {
        self.age += dt;
        if self.age > BULLET_LIFETIME {
            self.alive = false;
        }

        self.position += self.velocity * dt;

        // Wrap position to the screen
        if self.position.x.abs() > 1. { self.position.x *= -1.; }
        if self.position.y.abs() > 1. { self.position.y *= -1.; }
    }

    pub fn render<G, T>(&self, graphics: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize
    {
        polygon(
            RED,
            BULLET,
            math::identity()
                .trans(self.position.x, self.position.y)
                .scale(BULLET_SCALE, BULLET_SCALE),
            graphics,
        )
    }
}
