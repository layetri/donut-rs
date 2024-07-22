use crate::engine::clock::PPQ;

pub mod sequencer;

#[derive(Debug, Clone, Copy)]
pub struct Note {
    pub pitch: u8,
    pub velocity: f32,
    pub start: usize,
    pub duration: usize,
}

impl Note {
    pub fn quarter(pitch: u8) -> Self {
        Note {
            pitch,
            velocity: 1.0,
            start: 0,
            duration: PPQ
        }
    }
}

pub trait Generator {
    fn clear(&mut self);

    fn get_note_offs(&self) -> Vec<u8> {
        vec![]
    }

    fn pulse(&mut self, position: usize) -> Option<Note> {
        None
    }
    fn quarter(&mut self) -> Option<Note> {
        None
    }
    fn eighth(&mut self) -> Option<Note> {
        None
    }
    fn sixteenth(&mut self) -> Option<Note> {
        None
    }
}
