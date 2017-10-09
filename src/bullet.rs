use piston_window::{Graphics, ImageSize};
use cgmath::{Point2, Vector2, Basis2, Rotation2, Rotation, Rad};
use rustfest_game_assets::BULLET;

use transform::Transform;
use polygon::Polygon;

const BULLET_LIFETIME: f64 = 0.5;
const RED: [f32; 4] = [1., 0., 0., 1.];
const BULLET_SCALE: f64 = 0.01;
const BULLET_SPEED: f64 = 4.;

pub struct Bullet {
    pub alive: bool,
    pub transform: Transform,
    polygon: Polygon,
    velocity: Vector2<f64>,
    age: f64,
}

impl Bullet {
    pub fn new(position: Point2<f64>, rotation: Rad<f64>) -> Bullet {
        let direction = Basis2::from_angle(rotation).rotate_vector(Vector2::unit_y());
        Bullet {
            transform: Transform {
                position,
                rotation: Rad(0.),
                scale: Vector2 { x: BULLET_SCALE, y: BULLET_SCALE },
            },
            polygon: Polygon {
                color: RED,
                polygon: BULLET,
            },
            velocity: direction * BULLET_SPEED,
            age: 0.,
            alive: true,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.age += dt;
        if self.age > BULLET_LIFETIME {
            self.alive = false;
        }

        self.transform.position += self.velocity * dt;

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
