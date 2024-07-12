use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use cpal::BufferSize::Fixed;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::engine::midi::{MidiInputHandler, MidiMessage};
use crate::engine::synthesis::Synth;

pub enum AudioEngineControlPacket {
    NoteOn(u8, u8),
    NoteOff(u8),
    SetParameter(String, f32)
}

pub enum AudioEngineFeedbackPacket {

}

pub struct EngineManager {
    pub host: cpal::Host,
    pub device: cpal::Device,
    pub config: cpal::StreamConfig,
    pub stream: cpal::Stream,

    pub to_engine: Sender<AudioEngineControlPacket>,
    pub from_engine: Receiver<AudioEngineFeedbackPacket>
}

impl EngineManager {
    pub fn new() -> EngineManager {
        // Initialize cpal
        let (from_engine_tx, from_engine_rx) = channel();
        let (to_engine_tx, to_engine_rx) = channel();

        let host = cpal::default_host();
        let device = host.default_output_device().expect("Failed to find a default output device");

        println!("{}", host.id().name());
        println!("device: {}", device.name().unwrap());

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let config = device.default_output_config().unwrap().config();

        let sr = config.sample_rate.0 as f32;
        let buffer_size = match config.buffer_size {
            Fixed(size) => size as usize,
            _ => 512,
        };

        let cb = Arc::new(Mutex::new(AudioEngine::new(to_engine_rx, from_engine_tx, sr, buffer_size)));

        let stream = device.build_output_stream(&config, {
            let _cb = cb.clone();
            move |output: &mut [f32], info: &cpal::OutputCallbackInfo| {
                _cb.lock().unwrap().process(output, info);
            }
        }, err_fn, None).unwrap();

        stream.play().unwrap();

        EngineManager {
            host,
            device,
            config,
            stream,

            to_engine: to_engine_tx,
            from_engine: from_engine_rx
        }
    }
}

pub struct AudioEngine {
    pub incoming: Receiver<AudioEngineControlPacket>,
    pub outgoing: Sender<AudioEngineFeedbackPacket>,
    pub is_playing: bool,
    pub sample_position: usize,

    synth: Synth,
    midi: MidiInputHandler,

    pub sample_rate: f32,
    pub buffer_size: usize
}

impl AudioEngine {
    pub fn new(incoming: Receiver<AudioEngineControlPacket>, outgoing: Sender<AudioEngineFeedbackPacket>, sr: f32, bs: usize) -> AudioEngine {
        AudioEngine {
            incoming,
            outgoing,
            sample_position: 0,
            is_playing: false,

            synth: Synth::new(sr, bs),
            midi: MidiInputHandler::init().unwrap(),

            sample_rate: sr,
            buffer_size: bs
        }
    }

    pub fn process(&mut self, data: &mut [f32], info: &cpal::OutputCallbackInfo) {
        self.synth.set_block_size(data.len());
        self.buffer_size = data.len();
        
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

        for i in 0..self.buffer_size {
            data[i] = output[i];
        }

        self.sample_position += self.buffer_size;
    }
}