use piston_window::Graphics;

use player::Player;
use asteroid::{Asteroid, AsteroidCollision};
use bullets::Bullets;

const NUM_ASTEROIDS: usize = 3;
const INITIAL_LEVEL: usize = 2;

pub struct Asteroids {
    asteroids: Vec<Asteroid>
}

impl Asteroids {
    pub fn new() -> Asteroids {
        let mut asteroids = vec![];
        for _ in 0..NUM_ASTEROIDS {
            asteroids.push(Asteroid::new(INITIAL_LEVEL, None));
        }
        Asteroids { asteroids }
    }

    pub fn update(&mut self, player: &mut Player, bullets: &mut Bullets, dt: f64) {
        let mut asteroid_deletions = vec![];
        for (asteroid_index, asteroid) in &mut self.asteroids.iter_mut().enumerate() {
            match asteroid.update(player, bullets, dt) {
                AsteroidCollision::None => (),
                AsteroidCollision::Player => {
                    player.kill();
                }
                AsteroidCollision::Bullet { index } => {
                    asteroid_deletions.push(asteroid_index);
                    player.score();
                    bullets.delete(index);
                },
            }
        }

        // Delete in reverse so the indices don't get stuffed up
        asteroid_deletions.reverse();
        for deletion in asteroid_deletions {
            let deleted = self.asteroids.swap_remove(deletion);
            match deleted.level {
                0 => (),
                1 => {
                    self.asteroids.push(Asteroid::new(0, Some(deleted.transform.position)));
                    self.asteroids.push(Asteroid::new(0, Some(deleted.transform.position)));
                },
                2 => {
                    self.asteroids.push(Asteroid::new(1, Some(deleted.transform.position)));
                    self.asteroids.push(Asteroid::new(1, Some(deleted.transform.position)));
                    self.asteroids.push(Asteroid::new(INITIAL_LEVEL, None));
                },
                _ => unreachable!("Asteroid level was wrong!!!")
            }
        }
    }

    pub fn render<G>(&self, graphics: &mut G)
        where G: Graphics
    {
        for asteroid in &self.asteroids { asteroid.render(graphics); }
    }
}
