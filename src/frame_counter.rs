use std::time::Instant;
use kay::World;
use stagemaster::UserInterfaceID;

pub struct FrameCounter {
    last_frame: Instant,
    frame_time_ms: f32,
}

impl FrameCounter {
    pub fn new() -> FrameCounter {
        FrameCounter {
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

    pub fn print_fps(&self, ui: UserInterfaceID, world: &mut World) {
        let fps = (1000. / self.frame_time_ms) as u8;
        ui.add_debug_text(
            "Frame info".to_owned().into(),
            format!("FPS: {}, Frame time (ms): {}", fps, self.frame_time_ms).into(),
            [0., 0., 0., 0.5],
            false,
            world,
        );
    }
}
