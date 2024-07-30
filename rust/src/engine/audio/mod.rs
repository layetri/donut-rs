pub mod manager;
pub mod engine;
pub mod handler;

pub use manager::EngineManager;
pub use engine::AudioEngine;
pub use handler::AudioHandler;

use crate::{dsp::buffer::Buffer, system::{dev::DevInfo, parameter::ParameterID}};


#[derive(Debug)]
pub enum AudioEngineControlPacket {
    SetParameter(ParameterID, f32),

    AudioPacket(Buffer, usize),

    SetBlockSize(usize),
    SetMidiInput(usize),
    SetMidiOutput(String),
    SetAudioInput(String),
    SetAudioOutput(String),

    TogglePlayback,
    StopPlayback,
    ResetPlayback,

    NoteOn(u8, f32),
    NoteOff(u8)
}

#[derive(Debug)]
pub enum AudioEngineFeedbackPacket {
    Block(Buffer),
    DebugInfo(DevInfo),

    BlockSize(usize),
    Position(usize)
}