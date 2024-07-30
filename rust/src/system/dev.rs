use std::time::{Duration, Instant};
use serde_json::{json, Value};

// Times in us

#[derive(Debug, Clone)]
pub struct DevInfo {
    pub avg_cycle_time: usize,
    pub max_cycle_time: usize,
    pub block_size: usize,
    pub sample_rate: f32,
    pub allowed_cycle_time: usize,
    pub is_first_run: bool,
}

impl DevInfo {
    pub fn start(block_size: usize, sample_rate: f32) -> Self {
        DevInfo {
            avg_cycle_time: 0,
            max_cycle_time: 0,
            block_size,
            sample_rate,
            allowed_cycle_time: 0,
            is_first_run: true,
        }
    }

    pub fn update(&mut self, block_size: usize, sample_rate: f32, start: Instant) {
        if self.is_first_run {
            self.is_first_run = false;
            return;
        }

        let elapsed = start.elapsed().as_micros() as usize;
        self.avg_cycle_time = (self.avg_cycle_time + elapsed) / 2;

        if elapsed > self.max_cycle_time {
            self.max_cycle_time = elapsed;
        }

        self.block_size = block_size;
        self.sample_rate = sample_rate;
    }

    pub fn get_json(&self) -> String {
        json!({
            "avg_cycle_time": format!("{:?}", self.avg_cycle_time),
            "max_cycle_time": format!("{:?}", self.max_cycle_time),
            "block_size": self.block_size,
            "sample_rate": self.sample_rate,
            "allowed_cycle_time": format!("{:?}", self.allowed_cycle_time),
            "is_first_run": self.is_first_run,
        }).to_string()
    }
}