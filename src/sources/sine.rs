use std::f32::consts::PI;
use smallvec::{SmallVec, smallvec};
use uuid::Uuid;
use crate::dsp::buffer::Buffer;
use crate::dsp::util::mtof;
use crate::sources::AudioSource;
use crate::system::parameter::Parameter;

// #[derive(Send, Sync)]
pub struct Sine {
    id: Uuid,
    buffer: Buffer,
    frequency: f32,
    phase: f32,
    phase_step: f32,
    sample_rate: f32,
    block_size: usize,
}

impl Sine {
    pub fn new(sample_rate: f32, block_size: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            buffer: Buffer::new(block_size, "Sine".to_string()),
            frequency: 440.0,
            phase: 440.0 / sample_rate,
            phase_step: 0.0,
            sample_rate,
            block_size,
        }
    }
}

impl AudioSource for Sine {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn process(&mut self) {
        for i in 0..self.block_size {
            self.buffer[i] = (PI * 2.0 * self.phase).sin() * 0.5;
            self.phase += self.phase_step;

            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
        }
    }

    fn set_pitch(&mut self, midi_note: u8) {
        self.set_frequency(mtof(midi_note as f32));
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.phase_step = self.frequency / self.sample_rate;
    }

    fn set_block_size(&mut self, block_size: usize) {
        self.block_size = block_size;
        self.buffer = Buffer::new(block_size, "Sine".to_string());
    }

    fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }
    fn get_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    fn get_parameters(&self) -> SmallVec<[&Parameter; 16]> {
        smallvec![]
    }

    fn get_parameters_mut(&mut self) -> SmallVec<[&mut Parameter; 16]> {
        smallvec![]
    }
}