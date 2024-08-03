// EngineManager
// Creates and manages IO threads

use std::sync::atomic::AtomicU16;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use cpal::BufferSize::Fixed;
use cpal::{Devices, InputDevices, OutputDevices};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::api::commands::PacketFromEngine;
// use midir::{Ignore, MidiInput};
use crate::system::dev::DevInfo;
use crate::system::parameter::ParameterID;

use super::{AudioEngineControlPacket, AudioEngineFeedbackPacket, AudioEngine, AudioHandler};

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
    pub from_handler: Receiver<AudioEngineFeedbackPacket>,
    pub from_engine: Receiver<AudioEngineFeedbackPacket>
}

impl EngineManager {
    pub fn new() -> EngineManager {
        // Initialize cpal
        let (from_handler_tx, from_handler_rx) = channel();
        let (to_engine_tx, to_engine_rx) = channel();
        let (from_engine_tx, from_engine_rx) = channel();

        let host = cpal::default_host();
        
        let audio_ins = host.input_devices().unwrap();
        let audio_outs = host.output_devices().unwrap();
        
        let device = host.default_output_device().expect("Failed to find a default output device");

        println!("{}", host.id().name());
        println!("device: {}", device.name().unwrap());

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let mut config = device.default_output_config().unwrap().config();

        config.buffer_size = Fixed(512);

        let sr = config.sample_rate.0 as f32;
        let buffer_size = match config.buffer_size {
            Fixed(size) => size as usize,
            _ => 512,
        };

        let (cross_engine_tx, cross_engine_rx) = channel();

        let packets_in_pipe = Arc::new(AtomicU16::new(0));
        let _packets = packets_in_pipe.clone();

        let _process_thread = std::thread::spawn(move || {
            let mut engine = AudioEngine::new(sr, buffer_size, from_engine_tx);
            let mut position = 0;

            loop {
                if _packets.load(std::sync::atomic::Ordering::SeqCst) > 1 {
                    std::thread::sleep(std::time::Duration::from_micros(100));
                    continue;
                }

                while let Ok(packet) = to_engine_rx.try_recv() {
                    match packet {
                        AudioEngineControlPacket::SetParameter(id, value) => {
                            engine.set_parameter(id, value);
                        },
                        AudioEngineControlPacket::SetMidiInput(port) => {
                            engine.set_midi_device(port);
                        },
                        AudioEngineControlPacket::TogglePlayback => {
                            engine.toggle_playback();
                        },
                        AudioEngineControlPacket::SetBlockSize(size) => {
                            engine.set_block_size(size);
                        },
                        AudioEngineControlPacket::NoteOn(note, velocity) => {
                            engine.note_on(note, velocity);
                        },
                        AudioEngineControlPacket::NoteOff(note) => {
                            engine.note_off(note);
                        },
                        _ => {
                            println!("Unhandled packet: {:?}", packet);
                        }
                    }
                }

                let output = engine.process();
                cross_engine_tx.send(AudioEngineControlPacket::AudioPacket(output, engine.sample_position)).unwrap();

                _packets.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        });

        let cb = Arc::new(Mutex::new(AudioHandler::new(sr, buffer_size, packets_in_pipe, from_handler_tx, cross_engine_rx)));

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
            from_handler: from_handler_rx,
            from_engine: from_engine_rx
        };
        
        em.refresh_device_list();
        
        em
    }
    
    pub fn refresh_device_list(&mut self) {
        self.audio_ins = self.host.input_devices().unwrap();
        self.audio_outs = self.host.output_devices().unwrap();

        // let mut midi_in = MidiInput::new("Donut MIDI IN").unwrap();
        // midi_in.ignore(Ignore::None);

        // let in_ports = midi_in.ports();
        // in_ports.iter().enumerate().for_each(|(i, port)| {
        //     let name = midi_in.port_name(port).unwrap();
        //     self.midi_ins.push((name, i));
        // });
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
    
    pub fn note_on(&mut self, note: u8, velocity: f32) {
        self.to_engine.send(AudioEngineControlPacket::NoteOn(note, velocity)).unwrap();
    }
    
    pub fn note_off(&mut self, note: u8) {
        self.to_engine.send(AudioEngineControlPacket::NoteOff(note)).unwrap();
    }
    
    pub fn get_playback_status(&self) -> bool {
        self.playback_status
    }

    pub fn get_latest_debug_info(&self) -> DevInfo {
        self.latest_debug_info.clone()
    }

    pub fn run(&mut self) -> Vec<PacketFromEngine> {
        let mut packets = vec![];
        while let Ok(packet) = self.from_handler.try_recv() {
            match packet {
                AudioEngineFeedbackPacket::BlockSize(size) => {
                    self.config.buffer_size = Fixed(size as u32);
                    self.to_engine.send(AudioEngineControlPacket::SetBlockSize(size)).unwrap();
                },
                AudioEngineFeedbackPacket::Position(position) => {
                    packets.push(PacketFromEngine::Position(position));
                }
                _ => {}
            }
        }

        while let Ok(packet) = self.from_engine.try_recv() {
            match packet {
                AudioEngineFeedbackPacket::DebugInfo(info) => {
                    self.latest_debug_info = info.clone();
                    packets.push(PacketFromEngine::DebugInfo(info));
                },
                AudioEngineFeedbackPacket::Position(position) => {
                    packets.push(PacketFromEngine::Position(position));
                },
                AudioEngineFeedbackPacket::MidiPorts(ports) => {
                    self.midi_ins = ports.iter().enumerate().map(|(i, port)| (port.clone(), i)).collect();

                    packets.push(PacketFromEngine::MidiPorts(ports));
                },
                _ => {}
            }
        }

        packets
    }
}