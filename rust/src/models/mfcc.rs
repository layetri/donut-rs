use std::path::Path;
use crate::system::util::get_wavs_in_path;

pub struct MFCCModel {
    pub chunk_size: usize,
    pub n_filters: usize,
    pub n_files: usize,
    pub data: Vec<Vec<f32>>,
}

impl MFCCModel {
    pub fn train<T: AsRef<Path>>(path: T) -> Self {
        let files = get_wavs_in_path(path);
        
        let mut data = vec![];
        for file in files {
            let audio = hound::WavReader::open(file).unwrap();
            let samples = audio.into_samples::<f32>().map(|s| s.unwrap()).collect();
            data.push(samples);
        }
        
        
        
        
        MFCCModel {
            chunk_size: 1024,
            n_filters: 40,
            n_files: data.len(),
            data
        }
    }
    
    
}