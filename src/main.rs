use std::sync::{Arc, Mutex};
use crate::engine::audio::EngineManager;
use crate::gui::run_gui;

mod dsp;
mod system;

mod engine;
mod sources;
mod effects;
mod modulators;
mod gui;
mod models;

fn main() {
    // Start the audio thread
    let mut engine = Arc::new(Mutex::new(EngineManager::new()));

    run_gui(engine);

    // loop {
    //     // engine.run();
    //     std::thread::sleep(std::time::Duration::from_micros(100));
    // }
}
