use crate::dsp::buffer::Buffer;

struct Grain {
    pub start: usize,
    pub length: usize,
    pub buffer: Buffer,
}