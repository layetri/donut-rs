use std::sync::{atomic::AtomicU16, mpsc::{Receiver, Sender}, Arc};

use super::{AudioEngineControlPacket, AudioEngineFeedbackPacket};

pub struct AudioHandler {
    pub block_size: usize,
    pub sample_rate: f32,
    position: usize,

    pub packets_in_pipe: Arc<AtomicU16>,
    pub outgoing: Sender<AudioEngineFeedbackPacket>,
    pub incoming: Receiver<AudioEngineControlPacket>,
}

impl AudioHandler {
    pub fn new(sr: f32, bs: usize, qs: Arc<AtomicU16>, outgoing: Sender<AudioEngineFeedbackPacket>, incoming: Receiver<AudioEngineControlPacket>) -> AudioHandler {
        AudioHandler {
            block_size: bs,
            sample_rate: sr,
            packets_in_pipe: qs,
            position: 0,
            outgoing,
            incoming
        }
    }

    pub fn process(&mut self, data: &mut [f32], info: &cpal::OutputCallbackInfo) {
        if data.len() != self.block_size {
            self.block_size = data.len();
            self.outgoing.send(AudioEngineFeedbackPacket::BlockSize(self.block_size)).unwrap();
        }

        if let Ok(packet) = self.incoming.try_recv() {
            match packet {
                AudioEngineControlPacket::AudioPacket(buffer, start) => {
                    // if self.position <= start {
                        for i in 0..self.block_size {
                            data[i] = buffer[i];
                        }
                        self.outgoing.send(AudioEngineFeedbackPacket::Block(buffer)).unwrap();
                    // }
                    self.position += self.block_size;

                    self.packets_in_pipe.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                },
                _ => {}
            }
        }
    }
}