use std::{ops::Index, sync::Arc};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Div, IndexMut, Mul, MulAssign, Rem, Sub, SubAssign};
use std::path::Path;
use anyhow::{Result, Context};
use rubato::{Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction};
use crate::system::parameter::Parameter;

#[derive(Clone, Default)]
pub struct Buffer {
    size: usize,
    position: usize,
    data: Vec<f32>,
    multiplier: Option<Arc<Parameter>>,
    name: String
}

#[allow(dead_code)]
impl Buffer {
    pub fn new(length: usize, name: String) -> Self {
        Self {
            size: length,
            position: 0,
            data: vec![0.0; length],
            multiplier: None,
            name
        }
    }

    pub fn write(&mut self, value: f32) {
        self.data[self.position] = value;
    }

    pub fn read(&self) -> f32 {
        self.data[self.position]
    }

    pub fn write_ahead(&mut self, value: f32, places: isize) {
        let p = self.wrap_around(self.position as i64 + places as i64);
        self.data[p] = value;
    }

    pub fn write_addition(&mut self, value: f32) {
        self.data[self.position] += value;
    }

    pub fn write_at_position(&mut self, value: f32, position: usize) {
        self.data[position] = value;
    }

    pub fn tick(&mut self) {
        self.position = (self.position + 1) * (self.position < self.size-1) as usize;
    }

    pub fn flush(&mut self) {
        self.data[self.position] = 0.0;
    }

    pub fn wipe(&mut self) {
        for i in 0..self.size {
            self.data[i] = 0.0;
        }
    }

    pub fn set_position(&mut self, position: usize) {
        if position > 0 && position < self.size {
            self.position = position;
        }
    }

    pub fn attach_multiplier(&mut self, multiplier: Arc<Parameter>) {
        self.multiplier = Some(multiplier);
    }

    pub fn get_position(&self) -> usize {
        self.position
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_sample(&self, position: usize) -> Result<f32> {
        Ok(self.data[position])
    }
    pub fn get_current_sample(&self) -> f32 {
        self.data[self.position]
    }
    pub fn get_current_sample_multiplied(&self) -> Result<f32> {
        Ok(self.get_current_sample() * self.multiplier.clone().context("No multiplier attached")?.get_value())
    }
    pub fn read_ahead(&self, places: usize) -> Result<f32> {
        Ok(self.data[self.wrap_around(self.position as i64 + places as i64)])
    }
    pub fn read_back(&self, places: usize) -> Result<f32> {
        Ok(self.data[self.wrap_around(self.position as i64 - places as i64)])
    }

    pub fn get_multiplier(&self) -> Result<f32> {
        Ok(self.multiplier.clone().context("No multiplier attached")?.get_value())
    }

    fn wrap_around(&self, index: i64) -> usize {
        if index > 0 {
            index.rem(self.size as i64) as usize
        } else {
            let mut i = index;
            while i < 0 {
                i += self.size as i64;
            }

            i as usize
        }
    }

    pub fn as_vec(&self) -> Vec<f32> {
        self.data.clone()
    }
    pub fn from_vec(data: Vec<f32>) -> Self {
        Self {
            size: data.len(),
            position: 0,
            data,
            multiplier: None,
            name: String::default()
        }
    }
    
    pub fn from_file<T: AsRef<Path>>(path: T, sample_rate: f32) -> Result<Self> {
        let file = hound::WavReader::open(path).context("Failed to read buffer from file")?;
        let n_channels = file.spec().channels as usize;
        let source_sr = file.spec().sample_rate as f32;
        let fmt = file.spec().sample_format;
        let bits = file.spec().bits_per_sample;
        
        let mut data: Vec<f32> = if fmt == hound::SampleFormat::Int {
            match bits {
                16 => {
                    let data: Vec<i16> = file.into_samples::<i16>().map(|s| s.unwrap()).collect();
                    let mut f_data = vec![];
                    for i in 0..data.len() {
                        f_data.push(data[i] as f32 / i16::MAX as f32);
                    }
                    f_data
                },
                32 => {
                    let data: Vec<i32> = file.into_samples::<i32>().map(|s| s.unwrap()).collect();
                    let mut f_data = vec![];
                    for i in 0..data.len() {
                        f_data.push(data[i] as f32 / i32::MAX as f32);
                    }
                    f_data
                },
                _ => return Err(anyhow::anyhow!("Unsupported bit depth: {}", bits))
            }
        } else {
            file.into_samples::<f32>().map(|s| s.unwrap()).collect()
        };

        if n_channels > 1 {
            let mut mono = vec![];
            for i in 0..data.len() / n_channels {
                let mut sum = 0.0;
                for j in 0..n_channels {
                    sum += data[i * n_channels + j];
                }
                mono.push(sum / n_channels as f32);
            }
        }
        
        if source_sr.ne(&sample_rate) {
            let params = SincInterpolationParameters {
                sinc_len: 256,
                f_cutoff: 0.95,
                interpolation: SincInterpolationType::Linear,
                oversampling_factor: 256,
                window: WindowFunction::BlackmanHarris2,
            };
            let mut resampler = SincFixedIn::<f32>::new(
                sample_rate as f64 / source_sr as f64,
                2.0,
                params,
                data.len(),
                1,
            )?;

            let t = resampler.process(&[data], None)?;
            data = t[0].clone();
        }

        Ok(Self::from_vec(data))
    }
    
    pub fn get_between(&self, start: usize, end: usize) -> Vec<f32> {
        self.data[start..end].to_vec()
    }

    pub fn to_csv<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        let mut data = String::new();
        for i in 0..self.size {
            data.push_str(&self.data[i].to_string());
            if i < self.size - 1 {
                data.push(',');
            }
        }

        std::fs::write(path, data).context("Failed to write buffer to CSV")?;
        
        Ok(())
    }

    pub fn from_csv<T: AsRef<Path>>(path: T) -> Result<Self> {
        let data = std::fs::read_to_string(path).context("Failed to read buffer from CSV")?;
        let data: Vec<f32> = data.split(',').map(|s| s.parse().unwrap()).collect();

        Ok(Self::from_vec(data))
    }
    
    pub fn get_average(&self) -> f32 {
        let mut sum = 0.0;
        for i in 0..self.size {
            sum += self.data[i];
        }

        sum / self.size as f32
    
    }
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Buffer: {} ({} samples)", self.name, self.size)
    }

}

impl Add<&mut Buffer> for &Buffer {
    type Output = Buffer;

    fn add(self, rhs: &mut Buffer) -> Self::Output {
        assert_eq!(self.size, rhs.size, "Buffer sizes do not match");
        
        let mut data = self.clone();
        for i in 0..self.size {
            data[i] += rhs.data[i];
        }

        data
    }
}
impl Add<f32> for Buffer {
    type Output = Buffer;

    fn add(self, rhs: f32) -> Self::Output {
        let mut data = self.clone();
        for i in 0..self.size {
            data[i] += rhs;
        }

        data
    }
}
impl Sub<&mut Buffer> for &Buffer {
    type Output = Buffer;

    fn sub(self, rhs: &mut Buffer) -> Self::Output {
        assert_eq!(self.size, rhs.size, "Buffer sizes do not match");

        let mut data = self.clone();
        for i in 0..self.size {
            data[i] -= rhs.data[i];
        }

        data
    }
}
impl Sub<f32> for Buffer {
    type Output = Buffer;

    fn sub(self, rhs: f32) -> Self::Output {
        let mut data = self.clone();
        for i in 0..self.size {
            data[i] -= rhs;
        }
        
        data
    }
}
impl Index<usize> for Buffer {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.wrap_around(index as i64)]
    }
}
impl IndexMut<usize> for Buffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index % self.size]
    }
}
impl Mul<&Buffer> for &Buffer {
    type Output = Buffer;

    fn mul(self, rhs: &Buffer) -> Self::Output {
        assert_eq!(self.size, rhs.size, "Buffer sizes do not match");

        let mut data = self.clone();
        for i in 0..self.size {
            data[i] *= rhs.data[i];
        }
        data
    }
}
impl Mul<f32> for Buffer {
    type Output = Buffer;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut data = self.clone();
        for i in 0..self.size {
            data[i] *= rhs;
        }
        data
    }
}
impl Div<&mut Buffer> for &Buffer {
    type Output = Buffer;

    fn div(self, rhs: &mut Buffer) -> Self::Output {
        assert_eq!(self.size, rhs.size, "Buffer sizes do not match");

        let mut data = self.clone();
        for i in 0..self.size {
            data[i] /= rhs.data[i];
        }

        data
    }
}
impl Div<f32> for Buffer {
    type Output = Buffer;

    fn div(self, rhs: f32) -> Self::Output {
        let mut data = self.clone();
        for i in 0..self.size {
            data[i] /= rhs;
        }

        data
    }
}
impl AddAssign<Buffer> for Buffer {
    fn add_assign(&mut self, rhs: Buffer) {
        assert_eq!(self.size, rhs.size, "Buffer sizes do not match");

        for i in 0..self.size {
            self.data[i] += rhs.data[i];
        }
    }
}
// impl AddAssign<f32> for Buffer {
//     fn add_assign(&mut self, rhs: f32) {
//         for i in 0..self.size {
//             self.data[i] += rhs;
//         }
//     }
// }
impl SubAssign for Buffer {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.size, rhs.size, "Buffer sizes do not match");

        for i in 0..self.size {
            self.data[i] -= rhs.data[i];
        }
    }
}
impl SubAssign<f32> for Buffer {
    fn sub_assign(&mut self, rhs: f32) {
        for i in 0..self.size {
            self.data[i] -= rhs;
        }
    }
}
impl MulAssign for Buffer {
    fn mul_assign(&mut self, rhs: Self) {
        assert_eq!(self.size, rhs.size, "Buffer sizes do not match");

        for i in 0..self.size {
            self.data[i] *= rhs.data[i];
        }
    }
}
impl MulAssign<&Buffer> for Buffer {
    fn mul_assign(&mut self, rhs: &Buffer) {
        assert_eq!(self.size, rhs.size, "Buffer sizes do not match");

        for i in 0..self.size {
            self.data[i] *= rhs.data[i];
        }
    }
}
impl MulAssign<f32> for Buffer {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..self.size {
            self.data[i] *= rhs;
        }
    }
}