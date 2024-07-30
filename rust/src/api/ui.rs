use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread;

use serde::Serialize;
use anyhow::Result;

use lazy_static::lazy_static;
use serde_json::json;
use crate::api::commands::PacketFromUI;
use crate::engine::audio::EngineManager;
use crate::frb_generated::StreamSink;

// Stream invocation
#[derive(Clone, Debug, Serialize)]
pub struct StreamSinkPacket {
    pub destination: String,
    pub content: String,
}

pub trait GuiSender: Send + 'static + Clone {
    fn send_message(&self, message: StreamSinkPacket) -> Result<()>;
}

impl GuiSender for StreamSink<StreamSinkPacket> {
    fn send_message(&self, message: StreamSinkPacket) -> Result<()> {
        self.add(message).expect("Failed to send message to UI");

        Ok(())
    }
}

lazy_static! {
    pub static ref SENDER: Mutex<Option<Sender<PacketFromUI>>> = Default::default();
    static ref ALREADY_LOADED: Mutex<bool> = Mutex::new(false);
}


// #[flutter_rust_bridge::frb(sync)]
pub fn run_handler_thread(sink: StreamSink<StreamSinkPacket>) {
    let (sender, receiver) = channel();

    let th = std::thread::spawn(move || {
        println!("Started the audio thread 🦀");
        let mut engine = EngineManager::new();

        loop {
            while let Ok(message) = receiver.try_recv() {
                match message {
                    PacketFromUI::StartPlayback() => {
                        engine.toggle_playback();
                    }
                    PacketFromUI::StopPlayback() => todo!(),
                    PacketFromUI::SetMidiDevice() => todo!(),
                    PacketFromUI::SetAudioDevice() => todo!(),
                    PacketFromUI::SetParameter(id, value) => {
                        engine.set_parameter(id, value);
                    }
                    
                    PacketFromUI::NoteOn(pitch, velocity) => {
                        engine.note_on(pitch, velocity);
                    }
                    PacketFromUI::NoteOff(pitch) => {
                        engine.note_off(pitch);
                    }
                }
            }

            let messages = engine.run();

            for message in messages {
                let res = sink.add(StreamSinkPacket {
                    destination: message.get_destination(),
                    content: message.get_content()
                });

                if res.is_err() {
                    sink.add(StreamSinkPacket {
                        destination: String::from("error"),
                        content: json!({
                            "value": format!("Sink error: failed to send packet. Message: {:?}", res.err())
                        }).to_string()
                    }).unwrap();
                }
            }


            std::thread::sleep(std::time::Duration::from_micros(100));
        }
    });

    *SENDER.lock().unwrap() = Some(sender);
}