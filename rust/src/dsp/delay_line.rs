use crate::dsp::buffer::Buffer;

pub struct DelayLine {
    x: Buffer,
    feedback: f32,
    delay_time: usize
}

impl DelayLine {
    pub fn new(buffer_size: usize, delay_time: usize) -> Self {
        Self {
            x: Buffer::new(buffer_size, String::from("DelayLine")),
            feedback: 0.0,
            delay_time
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let delay = self.x.read_back(self.delay_time).unwrap_or(0.0);
        let output = input + delay * self.feedback;
        self.x.write(output);

        output
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback;
    }

    pub fn set_delay_time(&mut self, delay_time: usize) {
        self.delay_time = delay_time;
    }
}