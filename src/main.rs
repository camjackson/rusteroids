extern crate core;
extern crate kay;
extern crate monet;
extern crate stagemaster;
extern crate compact;
#[macro_use]
extern crate compact_macros;

use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use kay::{Networking, World, ActorSystem, Actor};
use monet::{RendererID, RenderableID};
use monet::glium::glutin::WindowBuilder;
use stagemaster::UserInterfaceID;

mod game_object;
mod thing;

use game_object::{GameID, GameObjectID};
use thing::{Thing, ThingID};

fn main() {
    // Initialise actor system
    let actor_system = &mut actor_system();
    let world = &mut actor_system.world();
    actor_system.networking_connect();

    // Initialise actor types
    thing::setup(actor_system);
    game_object::setup(actor_system);

    // Initialise game and graphics
    let game = game(world);
    let (ui, renderer) = graphics(actor_system);

    // Hello, actor system!
    let thing = ThingID::spawn(world);
    thing.do_something(42, world);

    // Process any initialisation messages
    actor_system.process_all_messages();

    // Loop forever!
    loop {
        game.update(0.1, world);

        actor_system.process_all_messages();
        actor_system.networking_send_and_receive();
        actor_system.networking_finish_turn();
    }
}

fn actor_system() -> ActorSystem {
    let machine_id = 0;
    let localhost_8080 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let networking = Networking::new(machine_id, vec!(localhost_8080));
    ActorSystem::new(networking)
}

fn game(world: &mut World) -> GameID {
    let game_object_broadcast_ids: Vec<GameObjectID> = vec![
        Thing::local_broadcast(world).into(),
    ].into();
    GameID::spawn(game_object_broadcast_ids.into(), world)
}

fn graphics(actor_system: &mut ActorSystem,) -> (UserInterfaceID, RendererID) {
    let renderable_broadcast_ids: Vec<RenderableID> = vec![

    ];
    let env = stagemaster::environment::Environment {
        name: "Rusteroids",
        version: "0.0.1",
        author: "Cam Jackson",
    };
    let window_builder = WindowBuilder::new()
        .with_title(env.name)
        .with_dimensions(1024, 768)
        .with_multitouch();
    let clear_color = (1., 0., 1., 1.0);
    stagemaster::setup(
        actor_system,
        renderable_broadcast_ids.into(),
        env,
        window_builder,
        clear_color,
    )
}
