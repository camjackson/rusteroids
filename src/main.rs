extern crate piston_window;
extern crate cgmath;
extern crate rustfest_game_assets;

use piston_window::*;
use cgmath::*;
use rustfest_game_assets::{PLAYER, BULLET, ASTEROIDS};

const ROTATION_SPEED: f64 = 2.;
const THRUST: f64 = 1.;

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
            let acceleration = Basis2::from_angle(player.rotation).rotate_vector(Vector2::unit_y());

            if controller.up { player.velocity += acceleration * THRUST * dt}
            if controller.down { player.velocity -= acceleration * THRUST * dt; }
            if controller.left { player.rotation += Rad(ROTATION_SPEED * dt); }
            if controller.right { player.rotation -= Rad(ROTATION_SPEED * dt); }

            player.position += player.velocity * dt;
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
