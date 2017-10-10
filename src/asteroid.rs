use piston_window::{Graphics, ImageSize};
use cgmath::{Point2, Vector2, Rad, InnerSpace};
use rustfest_game_assets::ASTEROIDS;
use rand::random;

use player::Player;
use transform::Transform;
use polygon::Polygon;
use bullets::Bullets;

const RED: [f32; 4] = [1., 0., 0., 1.];
const ASTEROID_SCALES: [f64; 3] = [0.03, 0.06, 0.08];
const ASTEROID_MAX_SPEED: f64 = 0.2;
const ASTEROID_MAX_SPIN: f64 = 1.;
const TAU: f64 = 6.283185;

// TODO: Don't spawn new asteroids on top of the player

pub struct Asteroid {
    pub level: usize,
    pub transform: Transform,
    polygon: Polygon,
    velocity: Vector2<f64>,
    spin: Rad<f64>,
}

pub enum AsteroidCollision {
    None,
    Player,
    Bullet { index: usize },
}

fn rand(min: f64, range: f64) -> f64 {
    random::<f64>() * range + min
}

impl Asteroid {
    pub fn new(level: usize, position: Option<Point2<f64>>) -> Asteroid {
        let actual_position = position.unwrap_or(Point2 { x: rand(-1., 2.), y: rand(-1., 2.) });
        let velocity = Vector2 {
            x: rand(-ASTEROID_MAX_SPEED, 2. * ASTEROID_MAX_SPEED),
            y: rand(-ASTEROID_MAX_SPEED, 2. * ASTEROID_MAX_SPEED),
        };
        Asteroid {
            transform: Transform {
                position: actual_position,
                rotation: Rad(rand(0., TAU)),
                scale: Vector2 { x: ASTEROID_SCALES[level], y: ASTEROID_SCALES[level] }
            },
            polygon: Polygon {
                color: RED,
                polygon: ASTEROIDS[(random::<f32>() * 5.) as usize],
            },
            velocity,
            spin: Rad(rand(0., ASTEROID_MAX_SPIN)),
            level: level,
        }
    }

    pub fn update(&mut self, player: &Player, bullets: &Bullets, dt: f64) -> AsteroidCollision {
        self.transform.position += self.velocity * dt;
        self.transform.rotation += self.spin * dt;

        // Wrap position to the screen
        if self.transform.position.x.abs() > 1. { self.transform.position.x *= -1.; }
        if self.transform.position.y.abs() > 1. { self.transform.position.y *= -1.; }

        // TODO: Non-shit collision detection

        for (bullet_index, bullet) in bullets.iter().enumerate() {
            let distance = (self.transform.position - bullet.transform.position).magnitude();
            if distance < self.transform.scale.x {
                return AsteroidCollision::Bullet { index: bullet_index };
            }
        }

        let player_distance = (self.transform.position - player.transform.position).magnitude();
        if player_distance < self.transform.scale.x + player.transform.scale.x {
           return AsteroidCollision::Player;
        }

        AsteroidCollision::None
    }

    pub fn render<G, T>(&self, graphics: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize
    {
        self.polygon.render(&self.transform, graphics);
    }
}
