pub struct KeyboardHalf {
    voices: Vec<usize>,
    rr_index: usize,
    name: String
}

pub struct NoteHandler {
    voices: Vec<usize>,
    used_voices: Vec<usize>,
    half_upper: KeyboardHalf,
    half_lower: KeyboardHalf
}