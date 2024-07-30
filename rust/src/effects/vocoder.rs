use std::sync::Arc;
use crate::dsp::biquad::{Biquad, BiquadShape};
use crate::dsp::buffer::Buffer;
use crate::dsp::fft::get_stft;
use crate::dsp::util::{ftom, mtof};

const VOCODER_BANDS: usize = 32;

pub struct Vocoder {
    filters: Vec<Biquad>,
    carrier: Arc<Buffer>,
    modulator: Arc<Buffer>,
    sample_rate: f32,
    block_size: usize,
}

impl Vocoder {
    pub fn new(sample_rate: f32, block_size: usize) -> Self {
        let mut filters = vec![];
        let bandwidth = sample_rate / (VOCODER_BANDS as f32 * 2.0);

        let min_midi = ftom(80.0);
        let max_midi = ftom(12000.0);

        let scale = |value| {
            (value / VOCODER_BANDS as f32) * (max_midi - min_midi) + min_midi
        };

        for i in 0..VOCODER_BANDS {
            let freq = scale(i as f32);
            filters.push(Biquad::new(freq, 7.628, 1.0, sample_rate, BiquadShape::Bandpass));
        }

        Self {
            filters,
            carrier: Arc::new(Buffer::new(block_size, String::default())),
            modulator: Arc::new(Buffer::new(block_size, String::default())),
            sample_rate,
            block_size
        }
    }

    pub fn process(&mut self) {
        let num_frames = (self.block_size as f32 / (VOCODER_BANDS as f32 / 2.0)).round() as usize;
        let stft = get_stft(&self.modulator, VOCODER_BANDS, num_frames);
        let window = Buffer::from_vec(apodize::hamming_iter(VOCODER_BANDS).map(|x| x as f32).collect::<Vec<f32>>());

        for (i, frame) in stft.iter().enumerate() {
            let slice = self.carrier.get_between(i*VOCODER_BANDS, (i+1)*VOCODER_BANDS);

            for (j, filter) in self.filters.iter_mut().enumerate() {
                filter.set_gain(frame[j].1);

                let mut _slice = Buffer::from_vec(slice.clone());
                _slice *= &window;
                filter.process_buffer(&mut _slice).unwrap()
            }
        }


    }
}