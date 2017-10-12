use std::any::Any;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

use kay;

fn actor_panic(_thing: Box<Any>, _world: &mut kay::World) {
    unimplemented!();
}

pub fn create() -> kay::ActorSystem {
    let machine_id = 0;
    let localhost_8080 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let networking = kay::Networking::new(machine_id, vec!(localhost_8080));
    kay::ActorSystem::new(Box::new(actor_panic), networking)
}
