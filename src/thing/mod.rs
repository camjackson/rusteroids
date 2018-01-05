use kay::{World, ActorSystem};

use game_object::{GameObject, GameObjectID};

#[derive(Compact, Clone)]
pub struct Thing {
    id: ThingID,
}

impl Thing {
    pub fn spawn(id: ThingID, _world: &mut World) -> Self {
        Thing { id }
    }

    pub fn do_something(&mut self, some_param: u8, _world: &mut World) {
        println!("DOING THE THING! {}", some_param);
    }
}

impl GameObject for Thing {
    fn update(&mut self, dt: f64, _: &mut World) {
        println!("UPDATING! {}", dt);
    }
}

pub fn setup(system: &mut ActorSystem) {
    system.register::<Thing>();

    auto_setup(system);
}

#[allow(dead_code)]
mod kay_auto;
pub use self::kay_auto::*;
