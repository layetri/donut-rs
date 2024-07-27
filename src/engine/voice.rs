use std::time::Instant;
use smallvec::{SmallVec, smallvec};
use uuid::Uuid;
use crate::dsp::biquad::{Biquad, BiquadShape};
use crate::dsp::buffer::Buffer;
use crate::modulators::adsr::ADSR;
use crate::modulators::Modulator;
use crate::sources::AudioSource;
use crate::sources::tensions::Tensions;
use crate::sources::waveshaper::WaveShaper;
use crate::sources::wavetable::WaveTable;
use crate::system::parameter::Parameter;
use crate::system::parameter::ParameterID::{KSAmount, WS1Amount, WT1Amount};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct VoiceData {
    pub sample_rate: f32,
    pub block_size: usize,
}

pub struct Voice {
    module_id: Uuid,

    // modulators: Vec<Box<dyn ModulationSource>>,
    sources: Vec<Box<dyn AudioSource + Send + Sync>>,
    effects: Vec<Box<dyn Modulator + Send + Sync>>,
    lpf: Biquad,
    envelope: ADSR,
    id: usize,
    data: VoiceData,

    levels: SmallVec<[Parameter; 16]>,

    midi_note: u8,
    is_busy: bool,
    last_used: Instant
}

impl Voice {
    pub fn new(id: usize, data: VoiceData) -> Self {
        println!("Creating voice {} with sr {}, buffer size {}", id, data.sample_rate, data.block_size);

        let module_id = Uuid::new_v4();

        let sources: Vec<Box<dyn AudioSource + Send + Sync>> = vec![
            Box::new(WaveShaper::new(data.sample_rate, data.block_size, id)),
            Box::new(Tensions::new(data.sample_rate, data.block_size, id)),
            Box::new(WaveTable::new(data.sample_rate, data.block_size, id))
        ];

        let mut levels: SmallVec<[Parameter; 16]> = smallvec![
            Parameter::from_id(WS1Amount, module_id, id, data.sample_rate),
            Parameter::from_id(KSAmount, module_id, id, data.sample_rate),
            Parameter::from_id(WT1Amount, module_id, id, data.sample_rate)
        ];

        for (i, p) in levels.iter_mut().enumerate() {
            p.assign_cc(41 + i as u8);
        }

        Voice {
            module_id,

            sources,
            effects: vec![],
            lpf: Biquad::new(6000.0, 0.707, 1.0, data.sample_rate, BiquadShape::Lowpass),
            envelope: ADSR::new(data.sample_rate, data.block_size, id),

            levels,
            
            id,
            data,

            midi_note: 0,
            is_busy: false,
            last_used: Instant::now()
        }
    }

    pub fn process(&mut self) -> Buffer {
        for source in &mut self.sources {
            source.process();
        }

        for effect in &mut self.effects {
            effect.process();
        }

        self.envelope.process();

        let mut output = Buffer::new(self.data.block_size, "Voice".to_string());
        let envelope_buffer = self.envelope.get_buffer();

        for (i, source) in self.sources.iter_mut().enumerate() {
            let source_buffer = source.get_buffer();
            output += envelope_buffer * source_buffer * self.levels[i].get_value();
        }
        
        self.lpf.process_buffer(&mut output).unwrap();

        output
    }

    pub fn note_on(&mut self, midi_note: u8, velocity: u8) {
        // println!("[{}] NoteOn: {} {}", self.id, midi_note, velocity);

        self.envelope.reset();
        self.envelope.start((velocity as f32 / 127.0).sqrt());

        self.last_used = Instant::now();
        self.is_busy = true;
        self.midi_note = midi_note;
        
        // self.lpf.set_cutoff(mtof(midi_note as f32) * 2.0);

        for source in &mut self.sources {
            source.set_pitch(midi_note);
        }
    }

    pub fn note_off(&mut self) {
        // println!("Voice {} received NoteOff for midi note {}", self.id, self.midi_note);

        self.envelope.stop();
        self.is_busy = false;
    }

    pub fn is_busy(&self) -> bool {
        self.is_busy
    }
    pub fn last_used(&self) -> Instant {
        self.last_used
    }
    pub fn get_midi_note(&self) -> u8 {
        self.midi_note
    }

    pub fn set_block_size(&mut self, block_size: usize) {
        self.data.block_size = block_size;
        for source in &mut self.sources {
            source.set_block_size(block_size);
        }

        self.envelope.set_block_size(block_size);
    }

    pub fn get_parameters(&mut self) -> SmallVec<[&Parameter; 64]> {
        let mut parameters = smallvec![];
        for source in self.sources.iter_mut() {
            let mut p = source.get_parameters();
            parameters.append(&mut p);
        }
        
        for l in self.levels.iter() {
            parameters.push(l);
        }

        let mut p = self.envelope.get_parameters();
        parameters.append(&mut p);

        parameters
    }

    pub fn get_parameters_mut(&mut self) -> SmallVec<[&mut Parameter; 64]> {
        let mut parameters = smallvec![];
        for source in self.sources.iter_mut() {
            let mut p = source.get_parameters_mut();
            parameters.append(&mut p);
        }
        
        for l in self.levels.iter_mut() {
            parameters.push(l);
        }

        let mut p = self.envelope.get_parameters_mut();
        parameters.append(&mut p);

        parameters
    }
}