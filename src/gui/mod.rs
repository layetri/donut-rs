mod startup;
mod midi;
mod mixer;
mod status;
mod controls;
mod devtools;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use imgui::{Condition, StyleVar};
use imgui::sys::{igGetWindowHeight, igGetWindowSize, igGetWindowWidth, ImGuiStyle, ImGuiWindowFlags, ImGuiWindowFlags_NoCollapse, ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoTitleBar};
use serde_json::json;
use crate::engine::audio::EngineManager;
use crate::gui::controls::ControlsWindow;
use crate::gui::midi::MidiWindow;
use crate::gui::status::StatusBar;

#[derive(Clone)]
pub struct WindowContext {
    pub width: f32,
    pub height: f32,
    pub engine: Arc<Mutex<EngineManager>>
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Window {
    MIDI,
    Audio,
    Oscilloscope,
    Spectrogram,
    Presets,
    Sampler,
    Pads,
    Controls,
    Modulation,
    Mixer,
    Devtools
}

pub fn run_gui(engine: Arc<Mutex<EngineManager>>) {
    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];

    let mut windows: HashMap<Window, bool> = [
        (Window::MIDI, true),
        (Window::Audio, false),
        (Window::Oscilloscope, false),
        (Window::Spectrogram, false),
        (Window::Presets, false),
        (Window::Sampler, false),
        (Window::Pads, false),
        (Window::Controls, true),
        (Window::Modulation, false),
        (Window::Mixer, true),
        (Window::Devtools, true)
    ].iter().cloned().collect();

    let mut state = json!({
       "parameters": {
            "WS1Detune": 440.0,
            "WS1Transpose": 0.0,
            "WS1Harmonics": 0.0,

            "WT1Shape": 0.0,
            "WT1Detune": 440.0,
            "WT1Transpose": 0.0,

            "ADSR1Attack": 20.0,
            "ADSR1Decay": 20.0,
            "ADSR1Sustain": 0.8,
            "ADSR1Release": 100.0,

            "KSCutoff": 10_000.0,
            "KSFeedback": 0.9999,

            "WT1Amount": 0.0,
            "WS1Amount": 0.0,
            "KSAmount": 1.0
        }
    });

    startup::simple_init("Donut 2", move |_, ui| {
        // let t1 = ui.push_style_var(StyleVar::ChildRounding(7.0));
        // let t2 = ui.push_style_var(StyleVar::FrameRounding(7.0));
        // let t3 = ui.push_style_var(StyleVar::GrabRounding(7.0));
        // let t4 = ui.push_style_var(StyleVar::TabRounding(7.0));
        // let t5 = ui.push_style_var(StyleVar::PopupRounding(7.0));
        // let t6 = ui.push_style_var(StyleVar::ScrollbarRounding(7.0));

        let ctx: WindowContext;
        unsafe {
            ctx = WindowContext {
                width: igGetWindowWidth(),
                height: igGetWindowHeight(),
                engine: engine.clone(),
            };
        }
        
        StatusBar::build(ui, ctx.clone(), &mut windows);
        
        if windows[&Window::MIDI] {
            MidiWindow::build(ui, ctx.clone());
        }

        if windows[&Window::Mixer] {
            mixer::MixerWindow::build(ui, ctx.clone(), &mut state);
        }
        
        if windows[&Window::Controls] {
            ControlsWindow::build(ui, ctx.clone(), &mut state);
        }

        if windows[&Window::Devtools] {
            devtools::DevToolsWindow::build(ui, ctx.clone(), &mut state);
        }
    });
}