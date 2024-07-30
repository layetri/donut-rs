pub mod api;
mod frb_generated;

use std::sync::{Arc, Mutex};
use crate::engine::audio::EngineManager;

mod dsp;
mod system;

mod engine;
mod sources;
mod effects;
mod modulators;
mod models;
mod generators;