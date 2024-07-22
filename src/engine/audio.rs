use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Instant;
use cpal::BufferSize::Fixed;
use cpal::{Devices, InputDevices, OutputDevices};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use midir::{Ignore, MidiInput, MidiInputPorts};
use crate::dsp::buffer::Buffer;
use crate::engine::midi::{MidiInputHandler, MidiMessage};
use crate::engine::synthesis::Synth;
use crate::system::dev::DevInfo;
use crate::system::parameter::ParameterID;

#[derive(Debug)]
pub enum AudioEngineControlPacket {
    SetParameter(ParameterID, f32),

    SetMidiInput(usize),
    SetMidiOutput(String),
    SetAudioInput(String),
    SetAudioOutput(String),

    TogglePlayback,
    StopPlayback,
    ResetPlayback
}

#[derive(Debug)]
pub enum AudioEngineFeedbackPacket {
    Block(Buffer),
    DebugInfo(DevInfo)
}

pub struct EngineManager {
    pub host: cpal::Host,
    pub device: cpal::Device,
    pub config: cpal::StreamConfig,
    pub stream: cpal::Stream,
    
    pub audio_ins: InputDevices<Devices>,
    pub audio_outs: OutputDevices<Devices>,
    pub midi_ins: Vec<(String, usize)>,
    pub active_midi_in: usize,
    pub playback_status: bool,
    pub latest_debug_info: DevInfo,

    pub to_engine: Sender<AudioEngineControlPacket>,
    pub from_engine: Receiver<AudioEngineFeedbackPacket>
}

impl EngineManager {
    pub fn new() -> EngineManager {
        // Initialize cpal
        let (from_engine_tx, from_engine_rx) = channel();
        let (to_engine_tx, to_engine_rx) = channel();

        let host = cpal::default_host();
        
        let audio_ins = host.input_devices().unwrap();
        let audio_outs = host.output_devices().unwrap();
        
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

        let mut em = EngineManager {
            host,
            device,
            config,
            stream,
            
            audio_ins,
            audio_outs,
            midi_ins: vec![],
            active_midi_in: 0,
            playback_status: false,
            latest_debug_info: DevInfo::start(buffer_size, sr),

            to_engine: to_engine_tx,
            from_engine: from_engine_rx
        };
        
        em.refresh_device_list();
        
        em
    }
    
    pub fn refresh_device_list(&mut self) {
        self.audio_ins = self.host.input_devices().unwrap();
        self.audio_outs = self.host.output_devices().unwrap();

        let mut midi_in = MidiInput::new("Donut MIDI IN").unwrap();
        midi_in.ignore(Ignore::None);

        let in_ports = midi_in.ports();
        in_ports.iter().enumerate().for_each(|(i, port)| {
            let name = midi_in.port_name(port).unwrap();
            self.midi_ins.push((name, i));
        });
    }
    
    pub fn get_midi_ports(&self) -> &Vec<(String, usize)> {
        &self.midi_ins
    }
    pub fn get_selected_midi_port(&self) -> usize {
        self.active_midi_in
    }
    
    pub fn get_audio_inputs(&self) -> &InputDevices<Devices> {
        &self.audio_ins
    }
    
    pub fn get_audio_outputs(&self) -> &OutputDevices<Devices> {
        &self.audio_outs
    }
    
    pub fn set_midi_device(&mut self, port: usize) {
        self.active_midi_in = port;
        self.to_engine.send(AudioEngineControlPacket::SetMidiInput(port)).unwrap();
    }
    
    pub fn set_parameter(&mut self, id: ParameterID, value: f32) {
        self.to_engine.send(AudioEngineControlPacket::SetParameter(id, value)).unwrap();
    }

    pub fn toggle_playback(&mut self) {
        self.playback_status = !self.playback_status;
        self.to_engine.send(AudioEngineControlPacket::TogglePlayback).unwrap();
    }

    pub fn stop_playback(&mut self) {
        self.playback_status = false;
        self.to_engine.send(AudioEngineControlPacket::StopPlayback).unwrap();
    }

    pub fn reset_playback(&mut self) {
        self.to_engine.send(AudioEngineControlPacket::ResetPlayback).unwrap();
    }

    pub fn get_playback_status(&self) -> bool {
        self.playback_status
    }

    pub fn get_latest_debug_info(&self) -> DevInfo {
        self.latest_debug_info.clone()
    }

    pub fn run(&mut self) {
        while let Ok(packet) = self.from_engine.try_recv() {
            match packet {
                AudioEngineFeedbackPacket::DebugInfo(info) => {
                    self.latest_debug_info = info;
                },
                _ => {
                    println!("Unhandled packet: {:?}", packet);
                }
            }
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
    pub dev_info: DevInfo,

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
            dev_info: DevInfo::start(bs, sr),


            sample_rate: sr,
            buffer_size: bs
        }
    }

    pub fn process(&mut self, data: &mut [f32], info: &cpal::OutputCallbackInfo) {
        let start = Instant::now();

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

        while let Ok(packet) = self.incoming.try_recv() {
            match packet {
                AudioEngineControlPacket::SetParameter(id, value) => {
                    self.synth.set_parameter(id, value);
                },
                AudioEngineControlPacket::SetMidiInput(port) => {
                    self.midi.set_input(port);
                },
                AudioEngineControlPacket::TogglePlayback => {
                    self.synth.toggle_playback();
                },
                _ => {
                    println!("Unhandled packet: {:?}", packet);
                }
            }
        }

        let output = self.synth.process();

        for i in 0..self.buffer_size {
            data[i] = output[i];
        }

        self.sample_position += self.buffer_size;
        self.outgoing.send(AudioEngineFeedbackPacket::Block(output)).unwrap();

        self.dev_info.update(self.buffer_size, self.sample_rate, start);
        self.outgoing.send(AudioEngineFeedbackPacket::DebugInfo(self.dev_info.clone())).unwrap();
    }
}