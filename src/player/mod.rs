use kay::{World, ActorSystem};

use game_object::{GameObject, GameObjectID};

#[derive(Compact, Clone)]
pub struct Player {
    id: PlayerID,
}

impl Player {
    pub fn spawn(id: PlayerID, _world: &mut World) -> Self {
        Player { id }
    }

    pub fn do_something(&mut self, some_param: u8, _world: &mut World) {
        println!("DOING THE THING! {}", some_param);
    }
}

impl GameObject for Player {
    fn update(&mut self, _dt: f64, _: &mut World) {
//        println!("UPDATING! {}", dt);
    }
}

pub fn setup(system: &mut ActorSystem) {
    system.register::<Player>();

    auto_setup(system);
}

#[allow(dead_code)]
mod kay_auto;
pub use self::kay_auto::*;
