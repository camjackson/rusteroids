use piston_window::{polygon, math, Transformed, Graphics, ImageSize};
use cgmath::{Basis2, Rotation2, Rotation, InnerSpace, Point2, Vector2, Rad};
use rustfest_game_assets::PLAYER;

use controller::Controller;
use bullets::Bullets;

const RED: [f32; 4] = [1., 0., 0., 1.];
const PLAYER_SCALE: f64 = 0.1;
const MAX_SPEED: f64 = 2.;
const ROTATION_SPEED: f64 = 2.;
const THRUST: f64 = 1.;
const FIRE_INTERVAL: f64 = 0.2;

pub struct Player {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    rotation: Rad<f64>,
    time_since_fired: f64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Point2 { x: 0., y: 0.  },
            velocity: Vector2 { x: 0., y: 0.  },
            rotation: Rad(0.),
            time_since_fired: 0.,
        }
    }

    pub fn update(&mut self, controller: &Controller, bullets: &mut Bullets, dt: f64) {
        let acceleration = Basis2::from_angle(self.rotation).rotate_vector(Vector2::unit_y())* THRUST;

        // Apply acceleration to the velocity
        if controller.up { self.velocity += acceleration * dt}
        if controller.down { self.velocity -= acceleration * dt; }
        if controller.left { self.rotation += Rad(ROTATION_SPEED * dt); }
        if controller.right { self.rotation -= Rad(ROTATION_SPEED * dt); }

        // Clamp velocity
        if self.velocity.magnitude() > MAX_SPEED {
            self.velocity = self.velocity.normalize_to(MAX_SPEED);
        }

        // Apply velocity to position
        self.position += self.velocity * dt;

        // Wrap position to the screen
        if self.position.x.abs() > 1. { self.position.x *= -1.; }
        if self.position.y.abs() > 1. { self.position.y *= -1.; }

        // Count time since last fire
        self.time_since_fired += dt;

        // Fire
        if controller.fire && self.time_since_fired > FIRE_INTERVAL {
            self.time_since_fired = 0.;
            bullets.spawn(self.position, self.rotation);
        }
    }

    pub fn render<G, T>(&self, graphics: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize
    {
        polygon(
            RED,
            PLAYER,
            math::identity()
                .trans(self.position.x, self.position.y)
                .scale(PLAYER_SCALE, PLAYER_SCALE)
                .rot_rad(self.rotation.0),
            graphics,
        );
    }
}
