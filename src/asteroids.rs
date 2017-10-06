use piston_window::{Graphics, ImageSize};

use asteroid::Asteroid;

const NUM_ASTEROIDS: usize = 3;

#[derive(Default)]
pub struct Asteroids {
    asteroids: Vec<Asteroid>
}

impl Asteroids {
    pub fn update(&mut self, dt: f64) {
        for asteroid in &mut self.asteroids {
            asteroid.update(dt);
        }

        if self.asteroids.len() < NUM_ASTEROIDS {
            self.asteroids.push(Asteroid::new())
        }
    }

    pub fn render<G, T>(&self, graphics: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize
    {
        for asteroid in &self.asteroids { asteroid.render(graphics); }
    }
}
