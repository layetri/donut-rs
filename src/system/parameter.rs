use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ParameterID {
    ADSR1Attack,
    ADSR1Decay,
    ADSR1Sustain,
    ADSR1Release,

    ADSR2Atttack,
    ADSR2Decay,
    ADSR2Sustain,
    ADSR2Release,

    FilterCutoff,
    FilterResonance,
    FilterType,
    FilterKeytrack,

    FMKeytrack,
    FMAmount,

    WT1Amount,
    WT1Shape,
    WT1Detune,
    WT1BaseFrequency,
    WT1Transpose,

    WT2Amount,
    WT2Shape,
    WT2Detune,
    WT2BaseFrequency,
    WT2Transpose,

    WS1Amount,
    WS1BaseFrequency,
    WS1Harmonics,
    WS1Detune,
    WS1DetuneRANGE,
    WS1Transpose,

    WS2Amount,
    WS2BaseFrequency,
    WS2Harmonics,
    WS2Detune,
    WS2DetuneRANGE,
    WS2Transpose,

    KSAmount,
    KSDelay,
    KSFeedback,
    KSCutoff,

    SAMPLERAmount,
    SAMPLERTranspose,
    SAMPLERBase,

    PARTICLESAmount,
    PARTICLESDensity,
    PARTICLESShape,
    PARTICLESAlgorithm,
    PARTICLESGrainSize,
    PARTICLESPosition,

    LFO1Rate,
    LFO1Sync,
    LFO2Rate,
    LFO2Sync,

    RND1Rate,
    RND1Range,
    RND1Sync,
    RND1Slew,

    FXDelayAmount,
    FXDelayTimeLeft,
    FXDelayTimeRight,
    FXDelayFeedbackLeft,
    FXDelayFeedbackRight,

    FXFoldAmount,
    FXFoldGain,

    OUTPUTHpfCutoff,
    VOICEMaster,
    INSTRUMENTMaster
}

#[derive(Copy, Clone)]
pub struct Parameter {
    id: ParameterID,
    voice_id: usize,
    value: f32,
    base_value: f32,
    min: f32,
    max: f32,
}

impl Parameter {
    pub fn get_value(&self) -> f32 {
        self.value
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParameterPreset {
    key: String,
    voice_id: usize,
    value: f32
}

