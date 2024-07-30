use rustfft::{FftPlanner, num_complex::Complex};
use crate::dsp::buffer::Buffer;

pub fn get_stft(buffer: &Buffer, fft_size: usize, num_frames: usize) -> Vec<Vec<(f32, f32)>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_size);
    
    let window: Vec<f32> = apodize::hamming_iter(fft_size).map(|x| x as f32).collect();
    
    let mut input = buffer.as_vec();
    let mut result = vec![];

    for i in 0..num_frames {
        let start = i * (fft_size / 2);
        let end = start + fft_size;
        let mut slice = input[start..end].to_vec();

        // Apply window
        for (s, w) in slice.iter_mut().zip(window.iter()) {
            *s *= *w;
        }
        
        let mut slice: Vec<Complex<f32>> = slice.iter().map(|x| Complex{ re: *x, im: 0.0f32 }).collect();
        fft.process(&mut slice);
        
        // Convert output to cartesian coordinates
        let mut output = vec![];
        for o in slice.iter_mut() {
            output.push(o.to_polar());
        }
        
        result.push(output);
    }

    result
}