use core::slice::Iter;
use piston_window::{Graphics, ImageSize};
use cgmath::{Point2, Rad};

use bullet::Bullet;

#[derive(Default)]
pub struct Bullets {
    bullets: Vec<Bullet>
}

impl Bullets {
    pub fn update(&mut self, dt: f64) {
        for bullet in &mut self.bullets {
            bullet.update(dt);
        }
        self.bullets.retain(|bullet| { bullet.alive });
    }

    pub fn render<G, T>(&self, graphics: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize
    {
        for bullet in &self.bullets { bullet.render(graphics); }
    }

    pub fn spawn(&mut self, position: Point2<f64>, rotation: Rad<f64>) {
        self.bullets.push(Bullet::new(position, rotation))
    }

    pub fn delete(&mut self, index: usize) {
        self.bullets.swap_remove(index);
    }

    pub fn iter(&self) -> Iter<Bullet> {
        self.bullets.iter()
    }
}
