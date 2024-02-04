use std::{ops::Index, sync::Arc};
use anyhow::{Result, Context};

use crate::system::parameter::Parameter;

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
            data: vec![],
            multiplier: None,
            name
        }
    }

    pub fn write(&mut self, value: f32) {
        self.data[self.position] = value;
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
}