use std::time::{Instant, Duration};

#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub delta_time: Duration,
    pub last_time: Instant,
    pub time: f32,
}

impl Time {
    pub fn new() -> Self {
        Self {
            delta_time: Duration::from_secs(0),
            last_time: Instant::now(),
            time: 0.0,
        }
    }
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_time);
        self.last_time = now;
        self.time += self.delta_time.as_secs_f32();
    }
}
