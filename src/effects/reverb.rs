use uuid::Uuid;
use crate::dsp::allpass::AllpassFilter;
use crate::dsp::comb::CombFilter;
use crate::system::parameter::Parameter;

pub struct Reverb {
    module_id: Uuid,
    
    time: Parameter,
    // mix: Parameter,
    
    comb_filters: [CombFilter; 8],
    allpass_filters: [AllpassFilter; 4],
    
    sample_rate: f32,
    block_size: usize
}

impl Reverb {
    // pub fn new(module_id: Uuid, sample_rate: f32, block_size: usize) -> Self {
    //     let mut comb_filters = [
    //         CombFilter::new(0.5, 0.0297, sample_rate)
    //     ];
    //     let mut allpass_filters = [
    //         AllpassFilter::new(0.7, sample_rate),
    //     ];
    // }
}