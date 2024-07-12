use std::{ops::Index, sync::Arc};
use std::ops::{Add, AddAssign, Div, IndexMut, Mul, MulAssign, Sub, SubAssign};
use std::path::Path;
use std::slice::SliceIndex;
use anyhow::{Result, Context};

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
        self.data[(self.position as isize + places) as usize] = value;
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
        Ok(self.data[self.position + places])
    }
    pub fn read_back(&self, places: usize) -> Result<f32> {
        Ok(self.data[self.position - places])
    }

    pub fn get_multiplier(&self) -> Result<f32> {
        Ok(self.multiplier.clone().context("No multiplier attached")?.get_value())
    }

    fn wrap_around(&self, index: usize) -> usize {
        index % self.size
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
        &self.data[self.wrap_around(index)]
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