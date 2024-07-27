use serde::{Deserialize, Serialize};

const DONUT_VERSION: usize = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetModLink {
    pub amount: f32,
    pub destination: String,
    pub source: String,
    pub voice: usize
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetParameter {
    pub base_value: f32,
    pub key: String,
    pub value: f32,
    pub voice: usize
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetSample {
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetSamplerRegion {
    pub key_end: u8,
    pub key_start: u8,
    pub key_root: u8,
    pub mode: usize,
    pub sample: String,
    pub smp_end: usize,
    pub smp_start: usize
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonutVersion {
    pub value: usize
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub donut_version: DonutVersion,
    pub mod_links: Vec<PresetModLink>,
    pub parameters: Vec<PresetParameter>,
    pub sample_lib: Vec<PresetSample>,
    pub sampler_regions: Vec<PresetSamplerRegion>
}

impl Preset {
    pub fn new() -> Self {
        Preset {
            donut_version: DonutVersion { value: 0 },
            mod_links: vec![],
            parameters: vec![],
            sample_lib: vec![],
            sampler_regions: vec![]
        }
    }

    pub fn from_state(state: &serde_json::Value) -> Self {
        let mut preset = Preset::new();

        preset.donut_version.value = DONUT_VERSION;

        if let Some(mod_links) = state.get("mod_links") {
            for link in mod_links.as_array().unwrap() {
                let amount = link.get("amount").unwrap().as_f64().unwrap() as f32;
                let destination = link.get("destination").unwrap().as_str().unwrap().to_string();
                let source = link.get("source").unwrap().as_str().unwrap().to_string();
                let voice = link.get("voice").unwrap().as_u64().unwrap() as usize;

                preset.mod_links.push(PresetModLink { amount, destination, source, voice });
            }
        }

        if let Some(parameters) = state.get("parameters") {
            for parameter in parameters.as_array().unwrap() {
                let base_value = parameter.get("base_value").unwrap().as_f64().unwrap() as f32;
                let key = parameter.get("key").unwrap().as_str().unwrap().to_string();
                let value = parameter.get("value").unwrap().as_f64().unwrap() as f32;
                let voice = parameter.get("voice").unwrap().as_u64().unwrap() as usize;

                preset.parameters.push(PresetParameter { base_value, key, value, voice });
            }
        }

        if let Some(sample_lib) = state.get("sample_lib") {
            for sample in sample_lib.as_array().unwrap() {
                let name = sample.get("name").unwrap().as_str().unwrap().to_string();

                preset.sample_lib.push(PresetSample { name });
            }
        }

        if let Some(sampler_regions) = state.get("sampler_regions") {
            for region in sampler_regions.as_array().unwrap() {
                let key_end = region.get("key_end").unwrap().as_u64().unwrap() as u8;
                let key_start = region.get("key_start").unwrap().as_u64().unwrap() as u8;
                let key_root = region.get("key_root").unwrap().as_u64().unwrap() as u8;
                let mode = region.get("mode").unwrap().as_u64().unwrap() as usize;
                let sample = region.get("sample").unwrap().as_str().unwrap().to_string();
                let smp_end = region.get("smp_end").unwrap().as_u64().unwrap() as usize;
                let smp_start = region.get("smp_start").unwrap().as_u64().unwrap() as usize;

                preset.sampler_regions.push(PresetSamplerRegion { key_end, key_start, key_root, mode, sample, smp_end, smp_start });
            }
        }

        preset
    }
}