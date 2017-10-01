extern crate piston_window;
extern crate cgmath;
extern crate rustfest_game_assets;

use piston_window::*;
use cgmath::*;

mod player;
mod controller;
mod bullet;
mod asteroid;

use player::Player;
use controller::Controller;
use bullet::Bullet;

// Render constants
const BLUE: [f32; 4] = [0., 0., 1., 1.];

// Game constants
const BULLET_SPEED: f64 = 4.;
const FIRE_INTERVAL: f64 = 0.2;


fn main() {
    let mut window: PistonWindow = WindowSettings::new("WOW IT'S A GAME", [480, 480])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .expect("OpenGL can't be instantiated");

    let mut controller = Controller::default();
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
                bullets.push(Bullet::new(player.position, bullet_velocity));
            }
        });

        event.button(|ButtonArgs { button, state, .. }| {
            controller.update(button, state)
        });

        window.draw_2d(&event, |_, graphics| {
            clear(BLUE, graphics);
            player.render(graphics);
            for bullet in &bullets { bullet.render(graphics); }
        });
    }
}
