use kay::{World, ActorSystem};
use compact::CVec;

pub trait GameObject {
    fn update(&mut self, dt: f64, world: &mut World);
}

#[derive(Compact, Clone)]
pub struct Game {
    id: GameID,
    broadcast_ids: CVec<GameObjectID>,
}

impl Game {
    pub fn spawn(id: GameID, broadcast_ids: &CVec<GameObjectID>, _: &mut World) -> Self {
        Game { id, broadcast_ids: broadcast_ids.clone() }
    }

    pub fn update(&mut self, dt: f64, world: &mut World) {
        for broadcast_id in &self.broadcast_ids {
            broadcast_id.update(dt, world);
        }
    }
}

pub fn setup(actor_system: &mut ActorSystem) {
    actor_system.register::<Game>();

    auto_setup(actor_system);
}

#[allow(dead_code)]
mod kay_auto;
pub use self::kay_auto::*;
