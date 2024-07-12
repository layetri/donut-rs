use std::cell::RefCell;
use std::f32::consts::PI;
use std::sync::Arc;
use smallvec::{SmallVec, smallvec};
use uuid::Uuid;
use crate::dsp::buffer::Buffer;
use crate::dsp::util::mtof_detune;
use crate::sources::AudioSource;
use crate::system::parameter::Parameter;
use crate::system::parameter::ParameterID::{WS1Detune, WS1Harmonics};

const TWO_PI: f32 = PI * 2.0;

#[derive(Default)]
pub struct WaveShaper {
    module_id: Uuid,

    base_frequency: f32,
    frequency: f32,
    harmonics: Parameter,
    detune: Parameter,
    transpose: i32,
    n: Vec<f32>,
    
    phase: f32,
    phase_step: f32,
    buffer: Buffer,
    
    sample_rate: f32,
    buffer_size: usize
}

impl WaveShaper {
    pub fn new(sample_rate: f32, buffer_size: usize, voice_id: usize) -> Self {
        let mut n = vec![];
        for i in 0..16 {
            n.push(2.0 * (i as f32 + 1.0) - 1.0);
        }

        let module_id = Uuid::new_v4();

        let mut harmonics = Parameter::from_id(WS1Harmonics, module_id, voice_id, sample_rate);
        let mut detune = Parameter::from_id(WS1Detune, module_id, voice_id, sample_rate);
        
        harmonics.assign_cc(21);
        detune.assign_cc(22);

        Self {
            module_id,

            base_frequency: 440.0,
            harmonics,
            detune,
            n,
            
            sample_rate,
            buffer_size,
            
            buffer: Buffer::new(buffer_size, String::from("WaveShaper")),
            
            ..Default::default()
        }
    }

    fn get_harmonics(&self) -> i32 {
        (self.harmonics.get_value() * 16.0).round() as i32
    }
}

#[allow(clippy::comparison_chain)]
impl AudioSource for WaveShaper {
    fn get_id(&self) -> Uuid {
        self.module_id
    }

    fn process(&mut self) {
        self.buffer.wipe();
        
        for _ in 0..self.buffer_size {
            if self.get_harmonics() < 0 {
                for i in 0..-self.get_harmonics() as usize {
                    self.buffer.write_addition((TWO_PI * self.n[i] * self.phase).sin() / self.n[i].max(1.0));
                }
            } else if self.get_harmonics() > 0 {
                for i in 0..self.get_harmonics() {
                    let x = i as f32;
                    self.buffer.write_addition((TWO_PI * x * self.phase).sin() / x.max(1.0));
                }
            } else {
                self.buffer.write((TWO_PI * self.phase).sin());
            }

            self.phase += self.phase_step;
            if self.phase > 1.0 {
                self.phase -= 1.0;
            }
            
            self.buffer.tick();
        }
    }

    fn set_pitch(&mut self, midi_note: u8) {
        self.frequency = mtof_detune(midi_note as f32, 440.0 + self.detune.get_value());
        self.phase_step = self.frequency / self.sample_rate;
        self.base_frequency = self.frequency;
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.phase_step = self.frequency / self.sample_rate;
    }

    fn fm(&mut self, frequency: f32, amount: f32) {
        self.set_frequency((self.base_frequency * 2.0 * frequency.abs() * amount) + self.base_frequency);
    }

    fn set_block_size(&mut self, block_size: usize) {
        if block_size != self.buffer_size {
            self.buffer = Buffer::new(block_size, String::from("WaveShaper"));
            self.buffer_size = block_size;
        }
    }

    fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }

    fn get_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    fn get_parameters(&self) -> SmallVec<[&Parameter; 16]> {
        smallvec![&self.harmonics, &self.detune]
    }

    fn get_parameters_mut(&mut self) -> SmallVec<[&mut Parameter; 16]> {
        smallvec![&mut self.harmonics, &mut self.detune]
    }
}