use crate::engine::audio::EngineManager;

mod dsp;
mod system;

mod engine;
mod sources;
mod effects;
mod modulators;

fn main() {
    // Start the audio thread
    let engine = EngineManager::new();

    loop {
        // engine.run();
        std::thread::sleep(std::time::Duration::from_micros(100));
    }
}
