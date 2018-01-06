use kay::{World, ActorSystem};
use monet::{RendererID, Renderable, RenderableID, Geometry, Vertex, Instance};

use super::{Player, PlayerID};

const SQUARE_ID: u16 = 1;

impl Renderable for Player {
    fn setup_in_scene(&mut self, renderer: RendererID, scene_id: usize, world: &mut World) {
        renderer.add_batch(
            scene_id,
            SQUARE_ID,
            Geometry::new(
                vec![
                    Vertex { position: [-1., -1., 0.]},
                    Vertex { position: [-1.,  1., 0.]},
                    Vertex { position: [ 1.,  1., 0.]},
                    Vertex { position: [ 1., -1., 0.]},
                ],
                vec![0, 1, 2, 0, 2, 3]),
            world);
    }

    fn render_to_scene(&mut self, renderer: RendererID, scene_id: usize, frame: usize, world: &mut World) {
        renderer.add_instance(
            scene_id,
            SQUARE_ID,
            frame,
            Instance {
                instance_position: [0., 0., 0.],
                instance_direction: [1., 0.],
                instance_color: [0., 1., 0.],
            },
            world,
        )
    }
}

pub fn setup(actor_system: &mut ActorSystem) {
    auto_setup(actor_system);
}

#[allow(dead_code)]
mod kay_auto;
pub use self::kay_auto::*;
