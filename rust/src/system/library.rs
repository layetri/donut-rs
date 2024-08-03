use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
// use tqdm::tqdm;
use crate::dsp::buffer::Buffer;
use crate::system::util::{default_path, get_wavs_in_path};

const LIBRARY_DATA_VERSION: usize = 100;

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleLibraryHeader {
    pub version: usize,
    pub paths: Vec<PathBuf>
}

impl SampleLibraryHeader {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        SampleLibraryHeader {
            version: LIBRARY_DATA_VERSION,
            paths
        }
    }

    pub fn load() -> Self {
        let path = default_path().join("library.json");
        let file = std::fs::File::open(&path).unwrap();

        serde_json::from_reader(file).unwrap()
    }

    pub fn save(&self) {
        let path = default_path().join("library.json");
        let file = std::fs::File::create(&path).unwrap();

        serde_json::to_writer(file, &self).unwrap();
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Sample {
    #[serde(skip)]
    pub buffer: Buffer,
    pub pitch: f32,
    pub mfcc: Vec<f32>,
    pub rms: f32,
    pub centroid: f32,

    pub path: String
}

impl Sample {
    pub fn create<T: AsRef<Path>>(path: T, sample_rate: f32) -> Self {
        let buffer = Buffer::from_file(&path, sample_rate).unwrap();

        let pitch = 0.0;
        let mfcc = vec![];
        let rms = 0.0;
        let centroid = 0.0;

        let mut s = Sample {
            buffer,
            pitch,
            mfcc,
            rms,
            centroid,
            path: path.as_ref().to_str().unwrap().to_string()
        };

        s.analysis(sample_rate);

        s
    }

    pub fn load(&mut self, sample_rate: f32) {
        self.buffer = Buffer::from_file(&self.path, sample_rate).unwrap();
    }

    fn analysis(&mut self, sample_rate: f32) {
        let opt = rsworld_sys::HarvestOption {
            f0_floor: 80.0,
            f0_ceil: 4000.0,
            frame_period: 5.0,
        };

        // // Pitch
        let buff = self.buffer.as_vec().iter().map(|x| *x as f64).collect::<Vec<f64>>();
        let (t, f0) = rsworld::harvest(&buff, sample_rate as i32, &opt);

        let pitch = f0.iter().fold(0.0, |acc, x| acc + x) / f0.len() as f64;

        // MFCC
        // let mfcc =

        // RMS
        // let rms = self.buffer.rms();

        // Centroid

        self.pitch = pitch as f32;
        // self.rms = rms;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleLibrary {
    pub header: SampleLibraryHeader,
    pub samples: Vec<Sample>
}

impl SampleLibrary {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        SampleLibrary {
            header: SampleLibraryHeader::new(paths),
            samples: vec![]
        }
    }

    pub fn get_default_path() -> PathBuf {
        // DDC: Donut Data Container :)
        default_path().join("library.ddc")
    }

    pub fn load() -> Self {
        let path = SampleLibrary::get_default_path();
        if !path.exists() {
            let sample_path = default_path().join("samples");
            return SampleLibrary::new(vec![sample_path]);
        }

        let header = SampleLibraryHeader::load();
        if header.version != LIBRARY_DATA_VERSION {
            return SampleLibrary::new(header.paths);
        }

        let file = std::fs::File::open(&path).unwrap();
        let reader = std::io::BufReader::new(file);

        bincode::deserialize_from(reader).unwrap()
        // serde_json::from_reader(reader).unwrap()
    }

    pub fn save(&self) {
        self.header.save();

        let path = SampleLibrary::get_default_path();
        let file = std::fs::File::create(&path).unwrap();
        let writer = std::io::BufWriter::new(file);

        bincode::serialize_into(writer, &self).unwrap();
        // serde_json::to_writer(writer, &self).unwrap();
    }

    pub fn load_samples(&mut self, sample_rate: f32) {
        let paths = self.header.paths.clone();
        for path in paths.iter() {
            self.load_samples_from_path(path, sample_rate, true);
        }

        self.save();
    }

    fn load_samples_from_path(&mut self, path: &Path, sample_rate: f32, skip_existing: bool) {
        let files = get_wavs_in_path(path);

        for file in files {
            if skip_existing && self.samples.iter().any(|s| s.path == file.to_str().unwrap()) {
                continue;
            }

            let sample = Sample::create(file, sample_rate);
            self.samples.push(sample);
        }
    }
    
    pub fn get_active_for_pitch(&self, pitch: u8) -> Option<&Sample> {
        
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_library() {
        let src_dir = homedir::my_home().unwrap().unwrap().join("donut").join("samples");
        let mut lib = SampleLibrary::new(vec![src_dir]);
        lib.load_samples(44100.0);
        lib.save();

        let lib2 = SampleLibrary::load();
        assert_eq!(lib.samples.len(), lib2.samples.len());
    }
}