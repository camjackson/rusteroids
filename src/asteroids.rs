use piston_window::{Graphics, ImageSize};

use asteroid::{Asteroid, UpdateResult};
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
            asteroids.push(Asteroid::new(INITIAL_LEVEL));
        }
        Asteroids { asteroids }
    }

    pub fn update(&mut self, bullets: &Bullets, dt: f64) {
        let mut deletions = vec![];
        for (index, asteroid) in &mut self.asteroids.iter_mut().enumerate() {
            match asteroid.update(bullets, dt) {
                UpdateResult::Lived => (),
                UpdateResult::Died => deletions.push(index),
            }
        }

        // Delete in reverse so the indices don't get stuffed up
        deletions.reverse();
        for deletion in deletions {
            let deleted = self.asteroids.swap_remove(deletion);
            match deleted.level {
                0 => (),
                1 => {
                    self.asteroids.push(Asteroid::new(0));
                    self.asteroids.push(Asteroid::new(0));
                },
                2 => {
                    self.asteroids.push(Asteroid::new(1));
                    self.asteroids.push(Asteroid::new(1));
                    self.asteroids.push(Asteroid::new(INITIAL_LEVEL));
                },
                _ => unreachable!("Asteroid level was wrong!!!")
            }
        }
    }

    pub fn render<G, T>(&self, graphics: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize
    {
        for asteroid in &self.asteroids { asteroid.render(graphics); }
    }
}
