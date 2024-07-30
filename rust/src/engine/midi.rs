use std::error::Error;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::borrow::Borrow;
// use midir::{Ignore, MidiInput, MidiInputConnection, MidiInputPort};
use anyhow::Result;

#[derive(Clone, Debug, PartialEq)]
pub enum MidiMessage {
    NoteOn(u8, u8),
    NoteOff(u8, u8),
    MidiCC(u8, u8),
    PitchBend(u8, u8),
    ModWheel(u8, u8),
    Unknown
}

#[derive(Clone, Debug, PartialEq)]
pub struct MidiInputCallbackInfo {
    pub timestamp: u64,
    pub message: MidiMessage
}

impl MidiMessage {
    pub fn from(data: &[u8]) -> Self {
        match data[0] {
            144 => MidiMessage::NoteOn(data[1], data[2]),
            128 => MidiMessage::NoteOff(data[1], data[2]),
            176 => MidiMessage::MidiCC(data[1], data[2]),
            224 => MidiMessage::PitchBend(data[1], data[2]),
            228 => MidiMessage::ModWheel(data[1], data[2]),
            _ => MidiMessage::Unknown
        }
    }
}

pub struct MidiInputHandler {
    // pub midi_in_port: MidiInputPort,
    // callbacks: Vec<Box<dyn FnMut(MidiInputCallbackInfo) + Send>>,
    // from_midi: Receiver<MidiInputCallbackInfo>,
    // midi_connection: MidiInputConnection<()>
}

impl MidiInputHandler {
    pub fn register_callback<F>(&mut self, callback: F) where F: FnMut(MidiInputCallbackInfo) + Send + 'static {
        // self.callbacks.push(Box::new(callback));
    }

    pub fn init() -> Result<Self, Box<dyn Error>> {
        // let mut midi_in = MidiInput::new("Donut MIDI IN")?;
        // midi_in.ignore(Ignore::None);

        // let in_ports = midi_in.ports();
        // let in_port = match in_ports.len() {
        //     0 => return Err("No MIDI inputs available".into()),
        //     1 => {
        //         println!("Only one port available. Donut uses this one automatically: {}",midi_in.port_name(&in_ports[0]).unwrap());
        //         &in_ports[0]
        //     },
        //     _ => {
        //         println!("Multiple devices found:");
        //         for port in in_ports.iter() {
        //             let pn = midi_in.port_name(port);
        //             println!("\t{:?}", pn);
        //         }

        //         // TODO: Add input selection
        //         &in_ports[0]
        //     }
        // };

        // let (tx, rx) = std::sync::mpsc::channel();

        // let _in = midi_in.connect(
        //     in_port,
        //     "donut-midi-in",
        //     move |stamp, message, _| {
        //         if message.len() == 3 {
        //             tx.send(MidiInputCallbackInfo {
        //                 timestamp: stamp,
        //                 message: MidiMessage::from(message)
        //             }).unwrap();
        //         }
        //     },
        //     ())?;

        Ok(Self {
            // midi_in_port: in_port.clone(),
            // callbacks: Vec::new(),
            // from_midi: rx,
            // midi_connection: _in
        })
    }
    
    pub fn set_input(&mut self, port: usize) {        
        // let mut midi_in = MidiInput::new("Donut MIDI IN").unwrap();
        // midi_in.ignore(Ignore::None);

        // let in_ports = midi_in.ports();
        // let in_port = &in_ports[port];

        // let (tx, rx) = std::sync::mpsc::channel();

        // let _in = midi_in.connect(
        //     in_port,
        //     "donut-midi-in",
        //     move |stamp, message, _| {
        //         if message.len() == 3 {
        //             tx.send(MidiInputCallbackInfo {
        //                 timestamp: stamp,
        //                 message: MidiMessage::from(message)
        //             }).unwrap();
        //         }
        //     },
        //     ()).unwrap();

        *self = Self {
            // midi_in_port: in_port.clone(),
            // callbacks: Vec::new(),
            // from_midi: rx,
            // midi_connection: _in
        };
    }

    pub fn run(&mut self) -> Vec<MidiMessage> {
        let mut messages = vec![];
        // while let Ok(msg) = self.from_midi.try_recv() {
            // messages.push(msg.message);
        // }

        messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_input() {
        // MidiInputHandler::run().unwrap();
    }
}