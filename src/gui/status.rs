use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use imgui::sys::{ImGuiWindowFlags_NoCollapse, ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoTitleBar};
use imgui::{Ui, WindowFlags};
use crate::engine::audio::EngineManager;

pub struct StatusBar;
impl StatusBar {
    pub fn build(ui: &mut Ui, engine: Arc<Mutex<EngineManager>>, windows: &mut HashMap<super::Window, bool>) {
        let mut topbar_flags = 0;
        topbar_flags |= ImGuiWindowFlags_NoResize;
        topbar_flags |= ImGuiWindowFlags_NoMove;
        topbar_flags |= ImGuiWindowFlags_NoCollapse;
        topbar_flags |= ImGuiWindowFlags_NoTitleBar;
        
        let flags = WindowFlags::from_bits(topbar_flags).unwrap();

        let is_playing: bool;
        {
            is_playing = engine.lock().unwrap().get_playback_status().clone();
        }
        
        ui.window("Application Controls")
            .flags(flags)
            .position([0.0, 0.0], imgui::Condition::Always)
            .size([ui.io().display_size[0], 45.0], imgui::Condition::Always)
            .build(|| {
                ui.text("Application Controls");
                
                for (window, visible) in windows.iter_mut() {
                    let name = format!("{:?}", window);
                    ui.same_line();
                    let clicked = ui.button(&name);
                    
                    if clicked {
                        *visible = !*visible;
                    }
                }

                ui.same_line();

                let play_button = ui.button(if is_playing { "Pause" } else { "Play" });
                if play_button {
                    engine.lock().unwrap().toggle_playback();
                }
            });
    }
}