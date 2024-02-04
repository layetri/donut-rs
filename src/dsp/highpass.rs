use super::{buffer::Buffer, filter::Filter};

pub struct HighPassFilter {
    pub delay_line: Buffer
}

impl Filter for HighPassFilter {
    
}