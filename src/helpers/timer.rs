use std::time::Instant;

pub struct Timer {
    start_time: Option<Instant>,
}

impl Timer {
    pub fn init() -> Self {
        Timer { start_time: None }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&mut self) -> f64 {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            self.start_time = None;
            elapsed.as_secs_f64()
        } else {
            0.0
        }
    }
}
