use smallvec::{SmallVec, smallvec};
use uuid::Uuid;
use crate::dsp::buffer::Buffer;
use crate::dsp::filter_delay_line::FilterDelayLine;
use crate::dsp::util::mtof;
use crate::sources::AudioSource;
use crate::system::parameter::Parameter;
use crate::system::parameter::ParameterID::{KSAmount, KSCutoff, KSDelay, KSFeedback};

const TRIGGER_TIME: u8 = 10;

#[derive(Default)]
pub struct Tensions {
    module_id: Uuid,
    voice_id: usize,

    buffer: Buffer,
    dl_in: Buffer,
    dl: FilterDelayLine,

    feedback: Parameter,
    dampening: Parameter,
    level: Parameter,

    frequency: f32,
    delay_time: f32,

    trigger_time: u8,

    sample_rate: f32,
    block_size: usize
}

impl Tensions {
    pub fn new(sample_rate: f32, block_size: usize, voice_id: usize) -> Self {
        let module_id = Uuid::new_v4();
        let mut feedback = Parameter::from_id(KSFeedback, module_id, voice_id, sample_rate);
        let mut dampening = Parameter::from_id(KSCutoff, module_id, voice_id, sample_rate);
        let mut level = Parameter::from_id(KSAmount, module_id, voice_id, sample_rate);

        feedback.assign_cc(21);
        dampening.assign_cc(22);

        Self {
            module_id,
            voice_id,

            buffer: Buffer::new(block_size, "Tensions".to_string()),
            dl_in: Buffer::new((sample_rate/4.0) as usize, "DL-IN".to_string()),
            dl: FilterDelayLine::new(sample_rate, block_size, 440.0, 10_000.0, 0.999999),

            feedback,
            dampening,
            level,
            sample_rate,
            block_size,

            frequency: 440.0,
            ..Default::default()
        }
    }

    pub fn sync(&mut self) {
        self.dl.set_cutoff(self.dampening.get_value());
        self.dl.set_delay_time(self.delay_time);
        self.dl.set_feedback(self.feedback.get_value());
    }

    fn excite(&mut self) {
        self.trigger_time = TRIGGER_TIME;

        for i in 0..TRIGGER_TIME as isize {
            self.dl_in.write_ahead((i % 2) as f32 * 2.0 - 1.0, i);
        }
    }
}

impl AudioSource for Tensions {
    fn get_id(&self) -> Uuid {
        self.module_id
    }

    fn get_name(&self) -> &str {
        "Tensions"
    }

    fn process(&mut self) {
        for i in 0..self.block_size {
            let x = self.dl.process_sample(self.dl_in.read());

            self.buffer.write(x);

            self.dl_in.tick();
            self.dl.tick();
            self.buffer.tick();

            if self.trigger_time > 0 {
                self.trigger_time -= 1;
            } else {
                self.dl_in.flush();
            }
        }

        // self.dl.process(&mut self.buffer);
    }

    fn set_pitch(&mut self, midi_note: u8) {
        self.delay_time = mtof(midi_note as f32);
        self.sync();

        self.excite();
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.delay_time = frequency;
        self.sync();

        self.excite();
    }

    fn set_block_size(&mut self, block_size: usize) {
        if block_size != self.block_size {
            self.buffer = Buffer::new(block_size, String::from("Tensions"));
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
        smallvec![&self.feedback, &self.dampening]
    }

    fn get_parameters_mut(&mut self) -> SmallVec<[&mut Parameter; 16]> {
        smallvec![&mut self.feedback, &mut self.dampening, &mut self.level]
    }

    fn get_level(&self) -> &Parameter {
        &self.level
    }

    fn get_level_mut(&mut self) -> &mut Parameter {
        &mut self.level
    }
}