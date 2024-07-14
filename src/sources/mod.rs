use std::cell::RefCell;
use std::sync::Arc;
use smallvec::SmallVec;
use uuid::Uuid;
use crate::dsp::buffer::Buffer;
use crate::system::parameter::Parameter;

pub mod sine;
pub mod wavetable;
pub mod waveshaper;
pub mod tensions;
pub mod granular;


pub trait AudioSource {
    fn get_id(&self) -> Uuid;
    
    fn process(&mut self);
    fn tick(&mut self) {}
    fn refresh(&mut self) {}
    fn set_pitch(&mut self, midi_note: u8);
    fn set_frequency(&mut self, frequency: f32);
    fn fm(&mut self, frequency: f32, amount: f32) {}
    fn set_block_size(&mut self, block_size: usize) {}

    fn get_buffer(&self) -> &Buffer;
    fn get_buffer_mut(&mut self) -> &mut Buffer;
    
    fn get_parameters(&self) -> SmallVec<[&Parameter; 16]>;
    fn get_parameters_mut(&mut self) -> SmallVec<[&mut Parameter; 16]>;
}