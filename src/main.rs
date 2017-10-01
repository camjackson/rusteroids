extern crate piston_window;
extern crate cgmath;
extern crate rustfest_game_assets;

use piston_window::*;
use cgmath::*;
use rustfest_game_assets::{PLAYER, BULLET, ASTEROIDS};

const ROTATION_SPEED: f64 = 2.;
const THRUST: f64 = 1.;
const MAX_VELOCITY: f64 = 2.;

#[derive(Default)]
struct ControllerState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

struct Player {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    rotation: Rad<f64>,
}

impl Player {
    fn update(&mut self, controller: &ControllerState, dt: f64) {
        let acceleration = Basis2::from_angle(self.rotation).rotate_vector(Vector2::unit_y());

        // Apply acceleration to the velocity
        if controller.up { self.velocity += acceleration * THRUST * dt}
        if controller.down { self.velocity -= acceleration * THRUST * dt; }
        if controller.left { self.rotation += Rad(ROTATION_SPEED * dt); }
        if controller.right { self.rotation -= Rad(ROTATION_SPEED * dt); }

        // Clamp velocity
        if self.velocity.magnitude() > MAX_VELOCITY {
            self.velocity = self.velocity.normalize_to(MAX_VELOCITY);
        }

        // Apply velocity to position
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
    };

    while let Some(event) = window.next() {
        event.update(|&UpdateArgs { dt }| {

            player.update(&controller, dt);
        });

        event.button(|ButtonArgs { button, state, .. }| {
            use Button::*;

            match button {
                Keyboard(Key::W) => { controller.up = state == ButtonState::Press; },
                Keyboard(Key::A) => { controller.left = state == ButtonState::Press; },
                Keyboard(Key::S) => { controller.down = state == ButtonState::Press; },
                Keyboard(Key::D) => { controller.right = state == ButtonState::Press; },
                _ => (),
            };
        });

        window.draw_2d(&event, |_, graphics| {
           clear([0., 0., 1., 1.], graphics);
            polygon(
                [1., 0., 0., 1.],
                PLAYER,
                math::identity()
                    .trans(player.position.x, player.position.y)
                    .scale(0.1, 0.1)
                    .rot_rad(player.rotation.0),
                graphics,
            );
        });
    }
}
