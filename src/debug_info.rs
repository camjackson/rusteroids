use std::time::Instant;
use kay::{World, ActorSystem};
use stagemaster::UserInterfaceID;

pub struct DebugInfo {
    last_frame: Instant,
    frame_time_ms: f32,
}

impl DebugInfo {
    pub fn new() -> DebugInfo {
        DebugInfo {
            last_frame: Instant::now(),
            frame_time_ms: 0.,
        }
    }

    pub fn next_frame(&mut self) {
        let elapsed = self.last_frame.elapsed();
        self.frame_time_ms = (
            elapsed.as_secs() * 1_000 + elapsed.subsec_nanos() as u64 / 1_000_000
        ) as f32;
        self.last_frame = Instant::now();
    }

    pub fn print_debug_info(&self, actor_system: &ActorSystem, ui: UserInterfaceID, world: &mut World) {
        let fps = (1000. / self.frame_time_ms) as u8;
        ui.add_debug_text(
            "Frame info".to_owned().into(),
            format!("FPS: {}, Frame time (ms): {}", fps, self.frame_time_ms).into(),
            [0., 0., 0., 0.5],
            false,
            world,
        );
        ui.add_debug_text(
            "Number of actors".to_owned().into(),
            actor_system.get_instance_counts().into(),
            [0.0, 0.0, 0.0, 1.0],
            false,
            world,
        );
        ui.add_debug_text(
            "Networking turn".to_owned().into(),
            actor_system.networking_debug_all_n_turns().into(),
            [0.0, 0.0, 0.0, 1.0],
            false,
            world,
        );
    }
}
