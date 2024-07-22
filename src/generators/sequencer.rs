use super::{Generator, Note};

pub struct Sequencer {
    pub is_playing: bool,
    pub position: usize,
    pub index: usize,

    pub pattern: Vec<Note>,

    pub held_notes: Vec<u8>
}

impl Sequencer {
    pub fn new() -> Self {
        Sequencer {
            is_playing: false,
            position: 0,
            index: 0,
            pattern: vec![
                Note::quarter(60),
                Note::quarter(62),
                Note::quarter(64),
                Note::quarter(65),
            ],
            held_notes: vec![]
        }
    }
}

impl Generator for Sequencer {
    fn quarter(&mut self) -> Option<Note> {
        // println!("Sequencer quarter note");
        
        let old_idx = self.index;
        self.index = (self.index + 1) % self.pattern.len();

        self.held_notes.push(self.pattern[old_idx].pitch);

        Some(self.pattern[self.index])
    }

    fn get_note_offs(&self) -> Vec<u8> {
        self.held_notes.clone()
    }

    fn clear(&mut self) {
        self.pattern = vec![
            Note::quarter(60),
            Note::quarter(62),
            Note::quarter(64),
            Note::quarter(65),
        ];
    }
}