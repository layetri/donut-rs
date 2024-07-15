use crate::dsp::buffer::Buffer;

pub struct CombFilter {
    feedback: f32,

    delay_buffer: Vec<f32>,
    delay_buffer_length: usize,
    delay_index: usize,

    sample_rate: f32
}

impl CombFilter {
    pub fn new(feedback: f32, delay: f32, sample_rate: f32) -> Self {
        let d = (delay * sample_rate) as usize + 1;

        CombFilter {
            feedback,
            delay_buffer: vec![0.0; d],
            delay_buffer_length: d,
            delay_index: 0,
            sample_rate
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let y = self.delay_buffer[self.delay_index];
        let out = y + self.feedback * input;

        self.delay_buffer[self.delay_index] = out;
        self.delay_index = (self.delay_index + 1) % self.delay_buffer_length;

        out
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback.clamp(0.0, 1.0);
    }

    pub fn set_delay(&mut self, delay: f32) {
        let d = (delay * self.sample_rate) as usize + 1;

        if d != self.delay_buffer_length {
            self.delay_buffer = vec![0.0; d];
            self.delay_buffer_length = d;
        }

        while self.delay_index >= d {
            self.delay_index -= d;
        }
    }
}