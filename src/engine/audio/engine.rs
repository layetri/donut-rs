// AudioEngine
// Handles audio processing callbacks and performs synthesis

use std::sync::mpsc::Sender;
use std::time::Instant;
use crate::dsp::buffer::Buffer;
use crate::engine::midi::{MidiInputHandler, MidiMessage};
use crate::engine::synthesis::Synth;
use crate::system::dev::DevInfo;
use crate::system::parameter::ParameterID;

use super::AudioEngineFeedbackPacket;

pub struct AudioEngine {
    // pub incoming: Receiver<AudioEngineControlPacket>,
    pub outgoing: Sender<AudioEngineFeedbackPacket>,
    pub is_playing: bool,
    pub sample_position: usize,

    synth: Synth,
    midi: MidiInputHandler,
    pub dev_info: DevInfo,

    pub sample_rate: f32,
    pub buffer_size: usize
}

impl AudioEngine {
    pub fn new(sr: f32, bs: usize, outgoing: Sender<AudioEngineFeedbackPacket>) -> AudioEngine {
        AudioEngine {
            sample_position: 0,
            is_playing: false,

            synth: Synth::new(sr, bs),
            midi: MidiInputHandler::init().unwrap(),
            dev_info: DevInfo::start(bs, sr),


            sample_rate: sr,
            buffer_size: bs,

            outgoing
        }
    }

    pub fn process(&mut self) -> Buffer {
        let start = Instant::now();
        
        let messages = self.midi.run();

        for message in messages {
            match message {
                MidiMessage::NoteOn(note, velocity) => {
                    self.synth.note_on(note, velocity);
                },
                MidiMessage::NoteOff(note, _) => {
                    self.synth.note_off(note);
                },
                MidiMessage::MidiCC(cc, value) => {
                    println!("Received midi CC message: {:?}", message);
                    self.synth.handle_cc(cc, value);
                },
                _ => {
                    println!("Unhandled message: {:?}", message);
                }
            }
        }

        let output = self.synth.process();

        self.sample_position += self.buffer_size;
        // self.outgoing.send(AudioEngineFeedbackPacket::Block(output)).unwrap();

        self.dev_info.update(self.buffer_size, self.sample_rate, start);
        self.outgoing.send(AudioEngineFeedbackPacket::DebugInfo(self.dev_info.clone())).unwrap();

        output
    }

    pub fn set_block_size(&mut self, block_size: usize) {
        if block_size == self.buffer_size {
            return;
        }

        println!("Setting block size to: {}", block_size);
        self.synth.set_block_size(block_size);
        self.buffer_size = block_size;
    }

    pub fn toggle_playback(&mut self) {
        self.is_playing = !self.is_playing;
        self.synth.toggle_playback();
    }

    pub fn set_parameter(&mut self, id: ParameterID, value: f32) {
        self.synth.set_parameter(id, value);
    }

    pub fn get_debug_info(&self) -> DevInfo {
        self.dev_info.clone()
    }

    pub fn set_midi_device(&mut self, port: usize) {
        self.midi.set_input(port);
    }
}