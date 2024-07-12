use std::f32::consts::PI;
use num_traits::Float;
use anyhow::Result;
use crate::dsp::buffer::Buffer;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BiquadShape {
    #[default]
    Lowpass,
    Highpass,
    Bandpass,
    Notch,
    Peak,
    Lowshelf,
    Highshelf
}

#[derive(Default)]
pub struct BiquadParams {
    pub a0: f32,
    pub a1: f32,
    pub a2: f32,
    pub b0: f32,
    pub b1: f32,
    pub b2: f32
}

#[derive(Default)]
pub struct Biquad {
    pub cutoff: f32,
    pub q: f32,
    pub gain: f32,
    pub sample_rate: f32,
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,

    pub c: BiquadParams,
    pub t: BiquadShape
}

impl Biquad {
    pub fn new(cutoff: f32, q: f32, gain: f32, sample_rate: f32, t: BiquadShape) -> Self {
        let mut b = Self {
            cutoff,
            q,
            gain,
            sample_rate,
            t,
            
            ..Default::default()
        };

        b.calculate_coefficients();

        b
    }

    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
        self.calculate_coefficients();
    }
    pub fn set_q(&mut self, q: f32) {
        self.q = q;
        self.calculate_coefficients();
    }
    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
        self.calculate_coefficients();
    }
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.calculate_coefficients();
    }
    pub fn set_shape(&mut self, t: BiquadShape) {
        self.t = t;
        self.calculate_coefficients();
    }

    pub fn process(&mut self, input: f32) -> f32 {
        self.c.b0 * input +
            self.c.b1 * self.x1 +
            self.c.b2 * self.x2 -
            self.c.a1 * self.y1 -
            self.c.a2 * self.y2
    }
    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }

    pub fn calculate_coefficients(&mut self) {
        let w0 = 2.0 * PI * self.cutoff / self.sample_rate;
        let alpha = 0.5 * w0.sin() / self.q;

        let a = 10.0.powf(self.gain / 40.0);
        let cos_omega = w0.cos();

        let (a0, a1, a2, b0, b1, b2) = match self.t {
            BiquadShape::Lowpass => {
                let b0 = (1.0 - cos_omega) / 2.0;
                let b1 = 1.0 - cos_omega;
                let b2 = b0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;
                (a0, a1, a2, b0, b1, b2)
            },
            BiquadShape::Highpass => {
                let b0 = (1.0 + cos_omega) / 2.0;
                let b1 = -1.0 * (1.0 + cos_omega);
                let b2 = b0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;
                (a0, a1, a2, b0, b1, b2)
            },
            BiquadShape::Bandpass => {
                let b0 = alpha;
                let b1 = 0.0;
                let b2 = -alpha;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;
                (a0, a1, a2, b0, b1, b2)
            },
            BiquadShape::Notch => {
                let b0 = 1.0;
                let b1 = -2.0 * cos_omega;
                let b2 = 1.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;
                (a0, a1, a2, b0, b1, b2)
            },
            BiquadShape::Peak => {
                let b0 = 1.0 + alpha * a;
                let b1 = -2.0 * cos_omega;
                let b2 = 1.0 - alpha * a;
                let a0 = 1.0 + alpha / a;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha / a;
                (a0, a1, a2, b0, b1, b2)
            },
            _ => {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
            }
        };

        self.c.b0 = (b0 / a0) * a;
        self.c.b1 = (b1 / a0) * a;
        self.c.b2 = (b2 / a0) * a;
        self.c.a1 = a1 / a0;
        self.c.a2 = a2 / a0;

        self.reset();
    }

    pub fn process_buffer(&mut self, buffer: &mut Buffer) -> Result<()> {
        for i in 0..buffer.get_size() {
            buffer[i] = self.process(buffer[i]);
        }

        Ok(())
    }
}