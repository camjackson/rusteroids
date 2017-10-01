extern crate piston_window;
extern crate cgmath;
extern crate rustfest_game_assets;

use piston_window::*;
use cgmath::*;
use rustfest_game_assets::{PLAYER, BULLET, ASTEROIDS};

// Render constants
const RED: [f32; 4] = [1., 0., 0., 1.];
const BLUE: [f32; 4] = [0., 0., 1., 1.];
const PLAYER_SCALE: f64 = 0.1;
const BULLET_SCALE: f64 = 0.02;

// Game constants
const ROTATION_SPEED: f64 = 2.;
const THRUST: f64 = 1.;
const MAX_SPEED: f64 = 2.;
const BULLET_SPEED: f64 = 4.;
const FIRE_INTERVAL: f64 = 0.2;
const BULLET_LIFETIME: f64 = 0.5;

#[derive(Default)]
struct ControllerState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    fire: bool,
}

impl ControllerState {
    fn update(&mut self, button: Button, state: ButtonState) {
        use Button::*;
        match button {
            Keyboard(Key::W) => { self.up = state == ButtonState::Press; },
            Keyboard(Key::A) => { self.left = state == ButtonState::Press; },
            Keyboard(Key::S) => { self.down = state == ButtonState::Press; },
            Keyboard(Key::D) => { self.right = state == ButtonState::Press; },
            Keyboard(Key::Space) => { self.fire = state == ButtonState::Press; }
            _ => (),
        };
    }
}

struct Player {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    rotation: Rad<f64>,
    time_since_fired: f64,
}

struct Bullet {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    alive: bool,
    age: f64,
}

struct Asteroid {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    rotation: Rad<f64>,
    level: u8,
}

impl Player {
    fn update(&mut self, controller: &ControllerState, dt: f64) {
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

impl Bullet {
    fn update(&mut self, _: &ControllerState, dt: f64) {
        self.age += dt;
        if self.age > BULLET_LIFETIME {
            self.alive = false;
        }

        self.position += self.velocity * dt;

        // Wrap position to the screen
        if self.position.x.abs() > 1. { self.position.x *= -1.; }
        if self.position.y.abs() > 1. { self.position.y *= -1.; }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("WOW IT'S A GAME", [480, 480])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .expect("OpenGL can't be instantiated");

    let mut controller = ControllerState::default();
    let mut player = Player {
        position: Point2 { x: 0., y: 0.  },
        velocity: Vector2 { x: 0., y: 0.  },
        rotation: Rad(0.),
        time_since_fired: 0.,
    };
    let mut bullets: Vec<Bullet> = vec![];

    while let Some(event) = window.next() {
        event.update(|&UpdateArgs { dt }| {
            player.update(&controller, dt);
            for bullet in &mut bullets {
                bullet.update(&controller, dt);
            }
            bullets.retain(|bullet| { bullet.alive });
            if controller.fire && player.time_since_fired > FIRE_INTERVAL {
                player.time_since_fired = 0.;
                let bullet_velocity = Basis2::from_angle(player.rotation).rotate_vector(Vector2::unit_y()) * BULLET_SPEED;
                bullets.push(Bullet {
                    position: player.position,
                    velocity: bullet_velocity,
                    age: 0.,
                    alive: true,
                });
            }
        });

        event.button(|ButtonArgs { button, state, .. }| {
            controller.update(button, state)
        });

        window.draw_2d(&event, |_, graphics| {
            clear(BLUE, graphics);
            polygon(
                RED,
                PLAYER,
                math::identity()
                    .trans(player.position.x, player.position.y)
                    .scale(PLAYER_SCALE, PLAYER_SCALE)
                    .rot_rad(player.rotation.0),
                graphics,
            );
            for bullet in &bullets {
                polygon(
                    RED,
                    BULLET,
                    math::identity()
                        .trans(bullet.position.x, bullet.position.y)
                        .scale(BULLET_SCALE, BULLET_SCALE),
                    graphics,
                )
            }
        });
    }
}
