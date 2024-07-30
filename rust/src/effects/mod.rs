use smallvec::SmallVec;
use uuid::Uuid;
use crate::dsp::buffer::Buffer;
use crate::system::parameter::Parameter;

pub mod vocoder;
pub mod reverb;
pub mod saturator;

pub trait AudioEffect {
    fn get_id(&self) -> Uuid;
    
    fn process(&mut self, input: &mut Buffer);
    
    fn get_buffer(&self) -> &Buffer;
    fn get_buffer_mut(&mut self) -> &mut Buffer;
    fn set_block_size(&mut self, block_size: usize);
    
    fn get_parameters(&self) -> SmallVec<[&Parameter; 16]>;
    fn get_parameters_mut(&mut self) -> SmallVec<[&mut Parameter; 16]>;
}