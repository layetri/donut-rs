use std::sync::{Arc, Mutex};
use smallvec::SmallVec;
use uuid::Uuid;
use crate::dsp::buffer::Buffer;
use crate::dsp::util::mtof;
use crate::sources::AudioSource;
use crate::system::library::{Sample, SampleLibrary};
use crate::system::parameter::Parameter;

pub struct Sampler {
    id: Uuid,
    voice_id: usize,
    name: String,

    pitch: u8,
    frequency: f32,

    library: Arc<Mutex<SampleLibrary>>,
    buffer: Buffer,

    block_size: usize,
    sample_rate: f32,
}

impl Sampler {
    pub fn new(library: Arc<Mutex<SampleLibrary>>, block_size: usize, sample_rate: f32, voice_id: usize) -> Sampler {
        let id = Uuid::new_v4();

        Sampler {
            id,
            voice_id,
            name: String::from("Sampler"),

            pitch: 0,
            frequency: 0.0,

            library,
            buffer: Buffer::new(block_size, "Sampler".to_string()),

            block_size,
            sample_rate,
        }
    }
}

impl AudioSource for Sampler {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn process(&mut self) {
        let sample = self.library.lock().unwrap().get_active_for_pitch(self.pitch);
        todo!()
    }

    fn set_pitch(&mut self, midi_note: u8) {
        self.pitch = midi_note;
        self.frequency = mtof(midi_note as f32);
    }

    fn set_frequency(&mut self, frequency: f32) {
        todo!()
    }

    fn get_buffer(&self) -> &Buffer {
        todo!()
    }

    fn get_buffer_mut(&mut self) -> &mut Buffer {
        todo!()
    }

    fn get_parameters(&self) -> SmallVec<[&Parameter; 16]> {
        todo!()
    }

    fn get_parameters_mut(&mut self) -> SmallVec<[&mut Parameter; 16]> {
        todo!()
    }

    fn get_level(&self) -> &Parameter {
        todo!()
    }

    fn get_level_mut(&mut self) -> &mut Parameter {
        todo!()
    }
}
