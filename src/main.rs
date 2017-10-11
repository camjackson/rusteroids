extern crate core;
extern crate piston_window;
extern crate cgmath;
extern crate rustfest_game_assets;
extern crate rand;
extern crate find_folder;
extern crate rodio;

use std::path::PathBuf;
use piston_window::*;

mod transform;
mod player;
mod controller;
mod bullets;
mod bullet;
mod asteroids;
mod asteroid;
mod polygon;

use player::Player;
use controller::Controller;
use bullets::Bullets;
use asteroids::Asteroids;

const BLUE: [f32; 4] = [0., 0., 1., 1.];
const GREEN: [f32; 4] = [0., 1., 0., 1.];

fn asset_path(path: &str) -> PathBuf {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    assets.join(path)
}

fn main() {
    // Create main window
    let mut window: PistonWindow = WindowSettings::new("WOW IT'S A GAME", [800, 800])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .expect("OpenGL can't be instantiated");

    // Create fonts
    let ref font = asset_path("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    // Initialise audio
    let audio_endpoint = rodio::get_default_endpoint().unwrap();

    // Create game objects
    let mut controller = Controller::default();
    let mut player = Player::new();
    let mut bullets = Bullets::default();
    let mut asteroids = Asteroids::new();

    // Loop forever!
    while let Some(event) = window.next() {
        event.update(|&UpdateArgs { dt }| {
            player.update(&audio_endpoint, &controller, &mut bullets, dt);
            bullets.update(dt);
            asteroids.update(&mut player, &mut bullets, dt);
        });

        event.button(|ButtonArgs { button, state, .. }| {
            controller.update(button, state)
        });

        window.draw_2d(&event, |context, graphics| {
            clear(BLUE, graphics);
            player.render(graphics);
            bullets.render(graphics);
            asteroids.render(graphics);
            let text_transform = context.transform.trans(700., 770.0);
            text::Text::new_color(GREEN, 32)
                .draw(
                    &player.score.to_string(),
                    &mut glyphs,
                    &context.draw_state,
                    text_transform,
                    graphics
                );
        });
    }
}
