use std::time::Instant;

pub struct Stats {
    pub fps: f64,
    pub tps: f64,
    pub last_frame: Instant,
    pub last_tick: Instant,
}
