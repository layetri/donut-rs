use std::env;

use super::commands::PacketFromUI;
use super::ui::SENDER;

pub use super::ui::run_handler_thread;

// #[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
// pub fn greet(name: String) -> String {
    // format!("Hello, {name}!")
// }

#[flutter_rust_bridge::frb(sync)]
pub fn play() {
    let message = PacketFromUI::StartPlayback();
    SENDER.lock().unwrap().as_ref().unwrap().send(message).unwrap();
}

#[flutter_rust_bridge::frb(sync)]
pub fn note_on(pitch: u8, velocity: f32) {
    let message = PacketFromUI::NoteOn(pitch, velocity);
    SENDER.lock().unwrap().as_ref().unwrap().send(message).unwrap();
}

#[flutter_rust_bridge::frb(sync)]
pub fn note_off(pitch: u8) {
    let message = PacketFromUI::NoteOff(pitch);
    SENDER.lock().unwrap().as_ref().unwrap().send(message).unwrap();
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    env::set_var("RUST_BACKTRACE", "1");
    flutter_rust_bridge::setup_default_user_utils();

    // run_handler_thread();
}