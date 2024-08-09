use serde_json::json;
use crate::system::dev::DevInfo;
use crate::system::parameter::ParameterID;

pub enum PacketFromUI {
    NoteOn(u8, f32),
    NoteOff(u8),

    StartPlayback(),
    StopPlayback(),
    SetMidiDevice(String),
    SetAudioDevice(),
    SetParameter(ParameterID, f32)
}

pub enum PacketFromEngine {
    Position(usize),
    Buffer(Vec<f32>),
    DebugInfo(DevInfo),
    MidiPorts(Vec<String>)
}

impl PacketFromEngine {
    pub fn get_destination(&self) -> &str {
        match self {
            Self::Position(..) => "engine.position",
            Self::Buffer(..) => "engine.buffer",
            Self::DebugInfo(..) => "debug",
            Self::MidiPorts(..) => "midi.ports"
        }
    }

    pub fn get_content(&self) -> String {
        match self {
            Self::Position(pos) => json!({
                "value": pos
            }).to_string(),
            Self::DebugInfo(info) => info.get_json(),
            Self::MidiPorts(ports) => json!({
                "ports": ports
            }).to_string(),
            _ => serde_json::Value::Null.to_string()
            // Self::Buffer(buffer) => buffer.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(", ")
        }
    }
}