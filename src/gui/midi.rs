use std::sync::{Arc, Mutex};
use imgui::{Condition, ListBox, Ui};
use crate::engine::audio::EngineManager;

pub struct MidiWindow;

impl MidiWindow {
    pub fn build(ui: &Ui, engine: Arc<Mutex<EngineManager>>) {
        let inputs;
        let mut midi_in_selector;
        
        {
            let e = engine.lock().unwrap();
            inputs = e.get_midi_ports().clone();
            midi_in_selector = e.get_selected_midi_port();
        }

        ui.window("MIDI")
            .size([350.0, 200.0], Condition::FirstUseEver)
            .build(|| {                
                ListBox::new("MIDI Inputs")
                    .build(ui, || {
                        for (name, i) in inputs.iter() {
                            let selected = *i == midi_in_selector;
                            if ui.selectable_config(name).selected(selected).build() {
                                midi_in_selector = *i;
                                engine.lock().unwrap().set_midi_device(*i);
                            }
                        }
                    });
            });
    }
}