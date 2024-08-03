use num_traits::Pow;
use smallvec::{SmallVec, smallvec};
use uuid::Uuid;
use crate::dsp::buffer::Buffer;
use crate::effects::AudioEffect;
use crate::system::parameter::{Parameter, ParameterID};

pub struct Saturator {
    id: Uuid,

    buffer: Buffer,

    alpha: Parameter,
    level: Parameter,

    block_size: usize,
    sample_rate: f32
}

impl Saturator {
    pub fn new(block_size: usize, sample_rate: f32) -> Saturator {
        let id = Uuid::new_v4();

        Saturator {
            id,

            buffer: Buffer::new(block_size, "Saturator".to_string()),
            alpha: Parameter::global_from_id(ParameterID::FXSaturatorAlpha, id, sample_rate),
            level: Parameter::global_from_id(ParameterID::FXSaturatorAmount, id, sample_rate),

            block_size,
            sample_rate
        }
    }
}

impl AudioEffect for Saturator {
    fn get_id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn process(&mut self, input: &mut Buffer) {
        let a = self.alpha.get_value();
        
        let wet = self.level.get_value();
        let dry = 1.0 - wet;

        for i in 0..input.get_size() {
            let mut x = (input[i] + 1.0) / 2.0;

            if x > 1.0 {
                x = (a + 1.0) / 2.0;
            } else if x > a {
                x = a + (x - a) / (1.0 + ((x - a) / (1.0 - a)).pow(2.0));
            }

            input[i] = ((x * 2.0) - 1.0 * wet) + (input[i] * dry);
        }
    }

    fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }

    fn get_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    fn set_block_size(&mut self, block_size: usize) {
        self.block_size = block_size;
    }

    fn get_parameters(&self) -> SmallVec<[&Parameter; 16]> {
        smallvec![&self.alpha]
    }

    fn get_parameters_mut(&mut self) -> SmallVec<[&mut Parameter; 16]> {
        smallvec![&mut self.alpha, &mut self.level]
    }
    
    fn get_level(&self) -> &Parameter {
        &self.level
    }
    
    fn get_level_mut(&mut self) -> &mut Parameter {
        &mut self.level
    }
}