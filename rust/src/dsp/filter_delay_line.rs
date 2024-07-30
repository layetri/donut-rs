use crate::dsp::biquad::Biquad;
use crate::dsp::buffer::Buffer;

#[derive(Default)]
pub struct FilterDelayLine {
    x: Buffer,
    y: Buffer,
    z: Buffer,
    feedback: f32,
    sample_rate: f32,
    dt: usize,
    cutoff: f32,

    filter: Biquad
}

impl FilterDelayLine {
    pub fn new(sample_rate: f32, buffer_size: usize, delay_time: f32, cutoff: f32, feedback: f32) -> Self {
        let dt = (sample_rate / delay_time).round() as usize;

        Self {
            x: Buffer::new(sample_rate as usize, String::from("FilterDelayLine-X")),
            y: Buffer::new(sample_rate as usize, String::from("FilterDelayLine-Y")),
            z: Buffer::new(sample_rate as usize, String::from("FilterDelayLine-Z")),
            feedback,
            dt,
            sample_rate,
            cutoff,
            filter: Biquad::lowpass(sample_rate, cutoff)
        }
    }

    pub fn process(&mut self, buffer: &mut Buffer) {
        self.x = buffer.clone();

        for i in 0..buffer.get_size() {
            buffer[i] = self.process_sample(buffer[i]);
            self.tick();
        }
    }

    pub fn process_sample(&mut self, sample: f32) -> f32 {
        self.x.write(sample);

        let x = sample + self.x.read_back(self.dt).unwrap() +
            ((self.y.read_back(self.dt + 1).unwrap() +
            self.y.read_back(self.dt).unwrap()) * 0.5) * self.feedback;
        self.y.write(x);

        let y = self.filter.process_out_of_place(&self.y, &self.z);

        self.z.write(self.y.read_back(self.dt).unwrap() + y);
        self.z.read()
    }

    pub fn tick(&mut self) {
        self.x.tick();
        self.y.tick();
        self.z.tick();
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback.clamp(0.1, 1.001);
    }

    pub fn set_delay_time(&mut self, delay_time: f32) {
        self.dt = (self.sample_rate / delay_time).round() as usize;
    }

    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
        self.filter.set_cutoff(cutoff);
    }
}