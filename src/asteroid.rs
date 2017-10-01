use cgmath::{Point2, Vector2, Rad};
use rustfest_game_assets::ASTEROIDS;

pub struct Asteroid {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    rotation: Rad<f64>,
    level: u8,
}
