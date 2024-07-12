use smallvec::{SmallVec, smallvec};
use uuid::Uuid;
use crate::dsp::buffer::Buffer;
use crate::modulators::Modulator;
use crate::system::parameter::Parameter;
use crate::system::parameter::ParameterID::{ADSR1Attack, ADSR1Decay, ADSR1Release, ADSR1Sustain};

#[derive(Debug, PartialEq, Default)]
enum ADSRState {
    Attack,
    Decay,
    Sustain,
    Release,
    #[default]
    Silence,
}

#[derive(Default)]
pub struct ADSR {
    module_id: Uuid,
    voice_id: usize,
    
    attack: Parameter,
    decay: Parameter,
    sustain: Parameter,
    release: Parameter,
    velocity: f32,

    sample_rate: f32,
    block_size: usize,
    buffer: Buffer,
    state: ADSRState,

    a_step: f32,
    d_step: f32,
    r_step: f32,
    value: f32,
    position: usize
}

impl ADSR {
    pub fn reset(&mut self) {
        self.state = ADSRState::Attack;
        self.position = 0;
        self.value = 0.0;

        self.buffer.flush();
    }

    pub fn new(sample_rate: f32, block_size: usize, voice_id: usize) -> Self {
        let mut res = Self::default();

        res.module_id = Uuid::new_v4();
        res.sample_rate = sample_rate;
        res.block_size = block_size;
        
        res.attack = Parameter::from_id(ADSR1Attack, res.module_id, voice_id, sample_rate);
        res.decay = Parameter::from_id(ADSR1Decay, res.module_id, voice_id, sample_rate);
        res.sustain = Parameter::from_id(ADSR1Sustain, res.module_id, voice_id, sample_rate);
        res.release = Parameter::from_id(ADSR1Release, res.module_id, voice_id, sample_rate);
        
        res.attack.assign_cc(25);
        res.decay.assign_cc(26);
        res.sustain.assign_cc(27);
        res.release.assign_cc(28);

        res.buffer = Buffer::new(block_size, "ADSR".to_string());
        res.value = 0.0;

        res
    }

    // pub fn set_attack(&mut self, attack: f32) {
    //     self.attack = ((attack / 1000.0) * self.sample_rate) as usize;
    //     self.a_step = 1.0 / self.attack as f32;
    // }
    // 
    // pub fn set_decay(&mut self, decay: f32) {
    //     self.decay = ((decay / 1000.0) * self.sample_rate) as usize;
    //     self.d_step = (1.0 - self.sustain) / self.decay as f32;
    // }
    // 
    // pub fn set_sustain(&mut self, sustain: f32) {
    //     self.sustain = sustain;
    // 
    //     self.d_step = (1.0 - self.sustain) / self.decay as f32;
    //     self.r_step = self.sustain / self.release as f32;
    // }
    // 
    // pub fn set_release(&mut self, release: f32) {
    //     self.release = ((release / 1000.0) * self.sample_rate) as usize;
    //     self.r_step = self.sustain / self.release as f32;
    // }
}

impl Modulator for ADSR {
    fn process(&mut self) {
        let a_step = 1.0 / self.attack.get_value();
        let d_step = (1.0 - self.sustain.get_value()) / self.decay.get_value();
        let r_step = self.sustain.get_value() / self.release.get_value();
        
        for i in 0..self.block_size {
            match self.state {
                ADSRState::Attack => {
                    self.value += a_step;
                    self.position += 1;

                    if self.position >= self.attack.get_value() as usize {
                        self.state = ADSRState::Decay;
                    }
                }
                ADSRState::Decay => {
                    self.value -= d_step;
                    self.position += 1;

                    if self.position >= (self.attack.get_value() + self.decay.get_value()) as usize {
                        self.state = ADSRState::Sustain;
                    }
                }
                ADSRState::Sustain => {
                    self.value = self.sustain.get_value();
                    self.position += 1;
                }
                ADSRState::Release => {
                    self.value -= r_step;
                    self.position += 1;

                    if self.position >= self.release.get_value() as usize {
                        self.state = ADSRState::Silence;
                    }
                }
                ADSRState::Silence => {
                    self.buffer.write(0.0);
                }
            }

            self.buffer[i] = self.value * self.velocity;
        }
    }

    fn set(&mut self, value: f32) {
        todo!()
    }

    fn start(&mut self, velocity: f32) {
        self.reset();
        self.velocity = velocity;
    }

    fn stop(&mut self) {
        self.state = ADSRState::Release;
        self.position = 0;
    }

    fn get(&self) -> f32 {
        todo!()
    }

    fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }

    fn get_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    
    fn set_block_size(&mut self, block_size: usize) {
        self.block_size = block_size;
        self.buffer = Buffer::new(block_size, "ADSR".to_string());
    }
    
    fn get_id(&self) -> Uuid {
        self.module_id
    }
    
    fn get_parameters(&self) -> SmallVec<[&Parameter; 16]> {
        smallvec![&self.attack, &self.decay, &self.sustain, &self.release]
    }
    
    fn get_parameters_mut(&mut self) -> SmallVec<[&mut Parameter; 16]> {
        smallvec![&mut self.attack, &mut self.decay, &mut self.sustain, &mut self.release]
    }
}