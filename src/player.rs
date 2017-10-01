use cgmath::*;

use controller::Controller;

const MAX_SPEED: f64 = 2.;
const ROTATION_SPEED: f64 = 2.;
const THRUST: f64 = 1.;

pub struct Player {
    pub position: Point2<f64>,
    pub velocity: Vector2<f64>,
    pub rotation: Rad<f64>,
    pub time_since_fired: f64,
}

impl Player {
    pub fn update(&mut self, controller: &Controller, dt: f64) {
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
    }
}
