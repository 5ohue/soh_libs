//-----------------------------------------------------------------------------
use std::time::Instant;
//-----------------------------------------------------------------------------

pub struct FrameTimer {
    last_tick: Instant,
    frame_num: usize,
}

impl FrameTimer {
    pub fn new() -> Self {
        return FrameTimer {
            last_tick: Instant::now(),
            frame_num: 0,
        };
    }

    pub fn frame_num(&self) -> usize {
        return self.frame_num;
    }

    /// Update frame timer returning time between this and last frame in seconds
    pub fn new_tick(&mut self) -> f32 {
        let now = Instant::now();

        let elapsed = self.last_tick.elapsed().as_secs_f32();

        self.last_tick = now;
        self.frame_num = self.frame_num.wrapping_add(1);

        return elapsed;
    }
}

impl Default for FrameTimer {
    fn default() -> Self {
        return Self::new();
    }
}

//-----------------------------------------------------------------------------
