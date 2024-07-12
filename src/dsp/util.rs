pub fn mtof(midi_note: f32) -> f32 {
    440.0 * 2.0f32.powf((midi_note - 69.0) / 12.0)
}

pub fn ftom(frequency: f32) -> f32 {
    (12.0 * (frequency / 440.0).log2() + 69.0)
}

pub fn mtof_detune(midi_note: f32, tune: f32) -> f32 {
    tune * 2.0f32.powf((midi_note - 69.0) / 12.0)
}