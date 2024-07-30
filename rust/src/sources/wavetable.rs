use std::env::home_dir;
use std::f32::consts::PI;
use std::path::PathBuf;
use smallvec::{SmallVec, smallvec};
use uuid::Uuid;
use crate::dsp::buffer::Buffer;
use crate::dsp::util::mtof_detune;
use crate::sources::AudioSource;
use crate::system::parameter::{Parameter, ParameterID};
use crate::system::parameter::ParameterID::{WT1Amount, WT1Detune, WT1Shape, WT1Transpose};

const TABLE_FREQUENCY: f32 = 1.0;
const BIT_DIV: f32 = 32768.0;

pub struct WaveTableLoader {
    data: Vec<Buffer>,

}

#[derive(Default)]
pub struct WaveTable {
    module_id: Uuid,
    sine: Buffer,
    square: Buffer,
    triangle: Buffer,

    prev_sine: f32,
    prev_square: f32,
    prev_triangle: f32,

    shape: Parameter,
    detune: Parameter,
    transpose: Parameter,
    level: Parameter,

    frequency: f32,

    mixer: f32,
    position: f32,

    sample_rate: f32,
    block_size: usize,
    buffer: Buffer
}

impl WaveTable {
    pub fn new(sample_rate: f32, block_size: usize, voice_id: usize) -> Self {
        let id = Uuid::new_v4();
        let mut shape = Parameter::from_id(WT1Shape, id, voice_id, sample_rate);
        let mut detune = Parameter::from_id(WT1Detune, id, voice_id, sample_rate);
        let transpose = Parameter::from_id(WT1Transpose, id, voice_id, sample_rate);
        let level = Parameter::from_id(WT1Amount, id, voice_id, sample_rate);

        shape.assign_cc(23);
        detune.assign_cc(24);
        
        let data_path = Self::get_data_path();
        if !data_path.exists() {
            std::fs::create_dir_all(&data_path).unwrap();
            Self::generate(sample_rate);
        }
        
        let sine_path = data_path.join("sine.csv");
        let square_path = data_path.join("square.csv");
        let triangle_path = data_path.join("triangle.csv");
        
        let sine = Buffer::from_csv(sine_path).unwrap();
        let square = Buffer::from_csv(square_path).unwrap();
        let triangle = Buffer::from_csv(triangle_path).unwrap();

        Self {
            module_id: id,
            
            sine,
            square,
            triangle,

            shape,
            detune,
            transpose,
            level,

            sample_rate,
            block_size,
            buffer: Buffer::new(block_size, "WaveTable".to_string()),

            ..Default::default()
        }
    }

    fn generate(sample_rate: f32) {
        let phase_step = 1.0 / sample_rate;
        let mut phase = 0.0;
        
        let mut sine = Buffer::new(sample_rate as usize, "sine".to_string());
        let mut square = Buffer::new(sample_rate as usize, "square".to_string());
        let mut triangle = Buffer::new(sample_rate as usize, "triangle".to_string());
        
        for i in 0..sample_rate as usize {
            sine[i] = (PI * 2.0 * phase).sin();
            square[i] = if phase < 0.5 { 1.0 } else { -1.0 };
            triangle[i] = if phase < 0.5 { 1.0 - 4.0 * phase } else { -1.0 + 4.0 * phase };
            
            phase += phase_step;
            if phase >= 1.0 {
                phase -= 1.0;
            }
        }
        
        let data_path = Self::get_data_path();
        let sine_path = data_path.join("sine.csv");
        let square_path = data_path.join("square.csv");
        let triangle_path = data_path.join("triangle.csv");
        
        sine.to_csv(sine_path).unwrap();
        square.to_csv(square_path).unwrap();
        triangle.to_csv(triangle_path).unwrap();
    }
    
    fn get_data_path() -> PathBuf {
        // TODO: Find a permanent solution for WASM
        // PathBuf::from("/home/layetri/donut/wavetable")
        homedir::my_home().unwrap().unwrap().join("donut").join("wavetable")
    }
}

impl AudioSource for WaveTable {
    fn get_id(&self) -> Uuid {
        self.module_id
    }
    fn get_name(&self) -> &str {
        "WaveTable"
    }

    fn process(&mut self) {
        
        self.mixer = 0.8 * self.mixer + 0.2 * self.shape.get_value();
        let mix_square = 1.0 - self.mixer.clamp(0.0, 1.0);
        let mix_sine = self.mixer.clamp(0.0, 1.0) - (self.mixer - 1.0).clamp(0.0, 1.0);
        let mix_triangle = (self.mixer - 1.0).clamp(0.0, 1.0);

        for i in 0..self.block_size {
            let p = self.position.floor() as usize;
            self.buffer[i] = 0.5 * (mix_square * (0.3 * self.square[p] + 0.7 * self.prev_square) +
                mix_triangle * (0.3 * self.triangle[p] + 0.7 * self.prev_triangle) +
                mix_sine * (0.3 * self.sine[p] + 0.7 * self.prev_sine));

            self.prev_square = self.square[p];
            self.prev_triangle = self.triangle[p];
            self.prev_sine = self.sine[p];

            self.position += self.frequency;
            if self.position > self.square.get_size() as f32 {
                self.position -= self.square.get_size() as f32;
            }
        }
    }

    fn set_pitch(&mut self, midi_note: u8) {
        self.frequency = (self.sample_rate / TABLE_FREQUENCY) / mtof_detune(127.0-(midi_note as f32 + 2.0 + self.transpose.get_value()), self.detune.get_value());
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = (self.sample_rate / TABLE_FREQUENCY) / frequency;
    }

    fn set_block_size(&mut self, block_size: usize) {
        if block_size != self.block_size {
            self.buffer = Buffer::new(block_size, String::from("WaveTable"));
            self.block_size = block_size;
        }
    }

    fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }

    fn get_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    fn get_parameters(&self) -> SmallVec<[&Parameter; 16]> {
        smallvec![&self.detune, &self.shape, &self.transpose]
    }

    fn get_parameters_mut(&mut self) -> SmallVec<[&mut Parameter; 16]> {
        smallvec![&mut self.detune, &mut self.shape, &mut self.transpose, &mut self.level]
    }

    fn get_level(&self) -> &Parameter {
        &self.level
    }

    fn get_level_mut(&mut self) -> &mut Parameter {
        &mut self.level
    }
}