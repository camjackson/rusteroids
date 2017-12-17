use kay::{World, ActorSystem};
use compact::CVec;

pub trait GameObject {
    fn update(&mut self, dt: f64, world: &mut World);
}

#[derive(Compact, Clone)]
pub struct Game {
    id: GameID,
    objects: CVec<GameObjectID>,
}

impl Game {
    pub fn spawn(id: GameID, objects: &CVec<GameObjectID>, _: &mut World) -> Self {
        Game { id, objects: objects.clone() }
    }

    pub fn update(&mut self, dt: f64, world: &mut World) {
        for game_object in &self.objects {
            game_object.update(dt, world);
        }
    }
}

pub fn setup(system: &mut ActorSystem) {
    system.register::<Game>();

    auto_setup(system);
}

#[allow(dead_code)]
mod kay_auto;
pub use self::kay_auto::*;
