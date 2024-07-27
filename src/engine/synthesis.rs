use crate::dsp::add_and_divide::AddAndDivide;
use crate::dsp::buffer::Buffer;
use crate::engine::voice::{Voice, VoiceData};
use crate::system::parameter::ParameterID;
use crate::engine::clock::Clock;

const VOICES: usize = 12;

pub struct Synth {
    pub voices: Vec<Voice>,
    pub sample_rate: f32,
    pub block_size: usize,

    voices_in_use: usize,
    next_voice: usize,
    sustained_notes: Vec<u8>,
    sustain: bool,
    mix: AddAndDivide,
    clock: Clock
}

impl Synth {
    pub fn new(sample_rate: f32, block_size: usize) -> Self {
        let data = VoiceData {
            sample_rate,
            block_size,
        };

        let mut voices = vec![];
        for v in 0..VOICES {
            voices.push(Voice::new(v, data.clone()));
        }

        Synth {
            voices,
            sample_rate,
            block_size,
            voices_in_use: 0,
            next_voice: 0,
            sustained_notes: vec![],
            sustain: false,
            mix: AddAndDivide::new(),
            clock: Clock::new(120.0, sample_rate, block_size)
        }
    }

    pub fn process(&mut self) -> Buffer {
        let _position = self.clock.tick();

        let note_ons = self.clock.get_notes();
        for note in note_ons {
            self.note_on(note.pitch, (note.velocity * 127.0) as u8);
        }

        let note_offs = self.clock.get_note_offs();
        for note in note_offs {
            self.note_off(note);
        }
        
        let mut outputs = vec![];
        for voice in &mut self.voices {
            outputs.push(voice.process());
        }

        self.mix.process(&outputs, self.block_size, self.voices_in_use as f32)
    }

    pub fn note_on(&mut self, midi_note: u8, velocity: u8) {
        let mut found = false;
        let old_idx = self.next_voice;
        
        while old_idx != (self.next_voice+1 % VOICES) {
            if self.voices[self.next_voice].get_midi_note() == midi_note {
                self.voices[self.next_voice].note_off();
            }
            if !self.voices[self.next_voice].is_busy() {
                self.voices[self.next_voice].note_on(midi_note, velocity);
                self.voices_in_use += 1;
                found = true;
                break;
            }
            
            self.next_voice = (self.next_voice + 1) % VOICES;
        }

        if !found {
            let mut oldest = 0;
            let mut oldest_time = self.voices[0].last_used();
            for (i, voice) in self.voices.iter().enumerate() {
                if voice.last_used() < oldest_time {
                    oldest = i;
                    oldest_time = voice.last_used();
                }
            }

            self.voices[oldest].note_on(midi_note, velocity);
        }
    }

    pub fn note_off(&mut self, midi_note: u8) {
        if self.sustain {
            self.sustained_notes.push(midi_note);
            return;
        }
        
        for voice in &mut self.voices {
            if voice.is_busy() && voice.get_midi_note() == midi_note {
                voice.note_off();
                self.voices_in_use -= 1;
                break;
            }
        }
    }
    
    pub fn handle_cc(&mut self, cc: u8, value: u8) {
        if cc == 64 {
            self.sustain = value == 127;
    
            if !self.sustain {
                let len = self.sustained_notes.len();
                for i in 0..len {
                    self.note_off(self.sustained_notes[i]);
                }
                self.sustained_notes.clear();
            }
        }
        
        for voice in self.voices.iter_mut() {
            let parameters = voice.get_parameters_mut();
            
            for parameter in parameters {
                if parameter.accepts_cc(cc) {
                    parameter.set_value(value as f32 / 127.0);
                }
            }
        }
    }

    pub fn toggle_playback(&mut self) -> bool {
        self.clock.toggle_play();
        self.clock.is_playing
    }
    
    pub fn set_parameter(&mut self, parameter: ParameterID, value: f32) {
        for voice in self.voices.iter_mut() {
            let parameters = voice.get_parameters_mut();
            
            'param: for p in parameters {
                if p.id == parameter {
                    p.set_value(value);
                    break 'param;
                }
            }
        }
    }

    pub fn set_block_size(&mut self, block_size: usize) {
        self.block_size = block_size;
        self.clock.set_block_size(block_size);

        for voice in &mut self.voices {
            voice.set_block_size(block_size);
        }
    }
}