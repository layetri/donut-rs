pub struct FilterCoefficients {
    pub a0: f32,
    pub a1: f32,
    pub a2: f32,
    pub b0: f32,
    pub b1: f32,
    pub b2: f32
}

pub trait Filter {
    fn fir(&mut self, delay: usize) -> f32 {
        0.0
    }

    fn iir(&mut self, delay: usize) -> f32 {
        0.0
    }
    
    fn tick(&mut self) {

    }

    fn get_sample(&self) -> f32 {
        0.0
    }
}