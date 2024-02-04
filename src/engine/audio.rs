use std::sync::mpsc::{Receiver, Sender};
use cpal::traits::{DeviceTrait, HostTrait};

pub enum AudioEngineControlPacket {
    NoteOn(u8, u8),
    NotOff(u8),
    SetParameter(String, f32)
}

pub enum AudioEngineFeedbackPacket {

}

pub struct AudioEngineManager {
    pub host: cpal::Host,
    pub device: cpal::Device,
    pub config: cpal::StreamConfig,
    pub stream: cpal::Stream,

    pub to_engine: Sender<AudioEngineControlPacket>,
    pub from_engine: Receiver<AudioEngineFeedbackPacket>
}

impl AudioEngineManager {
    pub fn new() -> AudioEngineManager {
        // Initialize cpal
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        let config = device.default_output_config().expect("no default output config available");

        // Create the audio engine
        let (to_engine_tx, to_engine_rx) = std::sync::mpsc::channel();
        let (from_engine_tx, from_engine_rx) = std::sync::mpsc::channel();

        let mut engine = AudioEngine {
            incoming: to_engine_rx,
            outgoing: from_engine_tx
        };

        // Create the audio stream
        let stream = device.build_output_stream(&config.into(), move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            engine.process(data);
        }, move |err| {
            eprintln!("an error occurred on stream: {}", err);
        }, None).expect("failed to build output stream");

        AudioEngineManager {
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
    pub outgoing: Sender<AudioEngineFeedbackPacket>
}

impl AudioEngine {
    pub fn new() -> AudioEngine {


        AudioEngine {}
    }

    pub fn process(&mut self, data: &mut [f32]) {

    }
}