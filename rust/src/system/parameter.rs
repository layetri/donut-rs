use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ParameterID {
    #[default]
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

    FXSaturatorAmount,
    FXSaturatorAlpha,

    FXFoldAmount,
    FXFoldGain,

    OUTPUTHpfCutoff,
    VOICEMaster,
    INSTRUMENTMaster
}

#[derive(Copy, Clone, Default)]
pub struct Parameter {
    pub(crate) id: ParameterID,
    module_id: Uuid,
    midi_id: u8,
    voice_id: Option<usize>,
    value: f32,
    base_value: f32,
    min: f32,
    max: f32,
}

impl Parameter {
    pub fn supported() -> &'static [ParameterID] {
        &[ParameterID::WS1Detune, ParameterID::WS1Harmonics, ParameterID::WT1Shape, ParameterID::WT1Detune, ParameterID::WT1Transpose, ParameterID::ADSR1Attack, ParameterID::ADSR1Decay, ParameterID::ADSR1Sustain, ParameterID::ADSR1Release, ParameterID::KSCutoff, ParameterID::KSFeedback]
    }

    pub fn from_id(id: ParameterID, module_id: Uuid, voice_id: usize, sample_rate: f32) -> Self {
        let ms = sample_rate / 1000.0;
        match id {
            ParameterID::WS1Detune => Self::new(id, module_id, Some(voice_id), 440.0, 440.0, (350.0, 500.0)),
            ParameterID::WS1Harmonics => Self::new(id, module_id, Some(voice_id), 0.0, 0.0, (-1.0, 1.0)),
            ParameterID::WT1Shape => Self::new(id, module_id, Some(voice_id), 0.0, 0.0, (0.0, 1.0)),
            ParameterID::WT1Detune => Self::new(id, module_id, Some(voice_id), 440.0, 440.0, (350.0, 500.0)),
            ParameterID::WT1Transpose => Self::new(id, module_id, Some(voice_id), 0.0, 0.0, (-12.0, 12.0)),
            
            ParameterID::ADSR1Attack => Self::new(id, module_id, Some(voice_id), 20.0*ms, 20.0*ms, (0.1*ms, 1000.0*ms)),
            ParameterID::ADSR1Decay => Self::new(id, module_id, Some(voice_id), 20.0*ms, 20.0*ms, (0.1*ms, 1000.0*ms)),
            ParameterID::ADSR1Sustain => Self::new(id, module_id, Some(voice_id), 0.8, 0.8, (0.0, 1.0)),
            ParameterID::ADSR1Release => Self::new(id, module_id, Some(voice_id), 100.0*ms, 100.0*ms, (0.1*ms, 1000.0*ms)),
            
            ParameterID::KSCutoff => Self::new(id, module_id, Some(voice_id), 10_000.0, 10_000.0, (1.0, 16_000.0)),
            ParameterID::KSFeedback => Self::new(id, module_id, Some(voice_id), 0.9999, 0.9999, (0.1, 0.99999999)),
            
            ParameterID::WT1Amount => Self::new(id, module_id, Some(voice_id), 0.5, 0.5, (0.0, 1.0)),
            ParameterID::WS1Amount => Self::new(id, module_id, Some(voice_id), 0.0, 0.0, (0.0, 1.0)),
            ParameterID::KSAmount => Self::new(id, module_id, Some(voice_id), 0.5, 0.5, (0.0, 1.0)),
            ParameterID::FXSaturatorAmount => Self::new(id, module_id, Some(voice_id), 0.5, 0.5, (0.0, 1.0)),

            _ => panic!("no parameter with that id")
        }
    }

    pub fn global_from_id(id: ParameterID, module_id: Uuid, sample_rate: f32) -> Self {
        match id {
            ParameterID::FXSaturatorAlpha => Self::new(id, module_id, None, 0.5, 0.5, (0.0, 1.0)),

            _ => panic!("no global parameter with that ID")
        }
    }
    
    pub fn new(id: ParameterID, module_id: Uuid, voice_id: Option<usize>, value: f32, base_value: f32, range: (f32, f32)) -> Self {
        Self {
            id,
            module_id,
            value,
            base_value,
            midi_id: 0,
            voice_id,
            min: range.0,
            max: range.1
        }
    }

    pub fn accepts(&self, id: &ParameterID) -> bool {
        id.eq(&self.id)
    }

    pub fn accepts_cc(&self, cc: u8) -> bool {
        cc.eq(&self.midi_id)
    }
    
    pub fn assign_cc(&mut self, cc: u8) {
        self.midi_id = cc;
    }
    
    pub fn get_value(&self) -> f32 {
        self.value
    }
    
    pub fn set_value(&mut self, value: f32) {
        self.value = (value * (self.max - self.min)) + self.min;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParameterPreset {
    key: String,
    voice_id: usize,
    value: f32
}