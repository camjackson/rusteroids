extern crate piston_window;
extern crate cgmath;
extern crate rustfest_game_assets;

use piston_window::*;

mod player;
mod controller;
mod bullets;
mod bullet;
mod asteroid;

use player::Player;
use controller::Controller;
use bullets::Bullets;

const BLUE: [f32; 4] = [0., 0., 1., 1.];

fn main() {
    let mut window: PistonWindow = WindowSettings::new("WOW IT'S A GAME", [480, 480])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .expect("OpenGL can't be instantiated");

    let mut controller = Controller::default();
    let mut player = Player::new();
    let mut bullets = Bullets::default();

    while let Some(event) = window.next() {
        event.update(|&UpdateArgs { dt }| {
            player.update(&controller, &mut bullets, dt);
            bullets.update(dt);
        });

        event.button(|ButtonArgs { button, state, .. }| {
            controller.update(button, state)
        });

        window.draw_2d(&event, |_, graphics| {
            clear(BLUE, graphics);
            player.render(graphics);
            bullets.render(graphics);
        });
    }
}
