use smallvec::SmallVec;
use uuid::Uuid;
use crate::dsp::buffer::Buffer;
use crate::system::parameter::Parameter;

pub mod adsr;

pub trait Modulator {
    fn process(&mut self);
    fn refresh(&mut self) {}
    fn tick(&mut self) {}
    fn set(&mut self, value: f32);

    fn sync(&mut self) {}
    fn sync_to_scheduler(&mut self) {}
    fn start(&mut self, velocity: f32) {}
    fn stop(&mut self) {}

    fn get(&self) -> f32;
    fn get_buffer(&self) -> &Buffer;
    fn get_buffer_mut(&mut self) -> &mut Buffer;
    fn set_block_size(&mut self, block_size: usize);
    
    fn get_id(&self) -> Uuid;
    fn get_parameters(&self) -> SmallVec<[&Parameter; 16]>;
    fn get_parameters_mut(&mut self) -> SmallVec<[&mut Parameter; 16]>;
}