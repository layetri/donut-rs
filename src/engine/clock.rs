use crate::generators::{sequencer::Sequencer, Generator, Note};

pub const PPQ: usize = 48;

pub struct Clock {
    pub bpm: f32,
    pub ppq: usize,
    pub position: usize,

    pub sample_rate: f32,
    pub sample_position: usize,
    pub is_playing: bool,
    pub block_size: usize,
    pub loop_point: usize,

    pub note_ons: Vec<Note>,

    pub generators: Vec<Box<dyn Generator + Send + Sync>>
}

impl Clock {
    pub fn new(bpm: f32, sample_rate: f32, block_size: usize) -> Self {
        Clock {
            bpm,
            ppq: PPQ,
            position: 0,
            sample_position: 0,
            is_playing: false,
            block_size,
            sample_rate,
            loop_point: 0,
            note_ons: vec![],
            generators: vec![
                Box::new(Sequencer::new())
            ]
        }
    }

    pub fn tick(&mut self) -> Option<usize> {
        if self.is_playing {
            self.sample_position += self.block_size;

            let old_pos = self.position;
            self.position = (self.sample_position as f32 / self.sample_rate * self.bpm / 60.0 * self.ppq as f32) as usize;
            
            if self.loop_point > 0 && self.position >= self.loop_point {
                self.position -= self.loop_point;
            }
            
            let has_passed_sixteenth = (old_pos as f32 / (self.ppq as f32 / 4.0)).floor() != (self.position as f32 / (self.ppq as f32 / 4.0)).floor();
            let has_passed_eighth = (old_pos as f32 / (self.ppq as f32 / 2.0)).floor() != (self.position as f32 / (self.ppq as f32 / 2.0)).floor();
            let has_passed_quarter = (old_pos as f32 / self.ppq as f32).floor() != (self.position as f32 / self.ppq as f32).floor();
            
            self.note_ons.clear();

            for generator in &mut self.generators {
                if has_passed_quarter {
                    if let Some(note) = generator.quarter() {
                        self.note_ons.push(note);
                    }
                }

                if has_passed_eighth {
                    if let Some(note) = generator.eighth() {
                        self.note_ons.push(note);
                    }
                }

                if has_passed_sixteenth {
                    if let Some(note) = generator.sixteenth() {
                        self.note_ons.push(note);
                    }
                }

                if let Some(note) = generator.pulse(self.position) {
                    self.note_ons.push(note);
                }
            }

            Some(self.position)
        } else {
            None
        }
    }

    pub fn get_notes(&self) -> Vec<Note> {
        self.note_ons.clone()
    }

    pub fn get_note_offs(&self) -> Vec<u8> {
        let mut note_offs = vec![];

        for generator in self.generators.iter() {
            let notes = generator.get_note_offs();
            note_offs.extend(notes);
        }

        note_offs
    }

    pub fn toggle_play(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn reset(&mut self) {
        self.position = 0;
        self.sample_position = 0;
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm;
    }

    pub fn set_ppq(&mut self, ppq: usize) {
        self.ppq = ppq;
    }

    pub fn set_block_size(&mut self, block_size: usize) {
        self.block_size = block_size;
    }
}