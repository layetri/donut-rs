use std::sync::{Arc, Mutex};
use imgui::{ChildWindow, Ui, VerticalSlider};
use crate::{engine::audio::EngineManager, system::dev::DevInfo};

pub struct DevToolsWindow;
impl DevToolsWindow {
    pub fn build(ui: &Ui, engine: Arc<Mutex<EngineManager>>, state: &mut serde_json::Value) {
        let m: DevInfo;
        {
            let mut e = engine.lock().unwrap();
            e.run();
            m = e.get_latest_debug_info();
        }

        ui.window("DevTools")
            .size([800.0, 240.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("Audio Engine Debug Info");

                ui.text(format!("Buffer Size: {}", m.block_size));
                ui.text(format!("Sample Rate: {} Hz", m.sample_rate));
                ui.text(format!("Average cycle time: {:?}", m.avg_cycle_time));
                ui.text(format!("Max cycle time: {:?}", m.max_cycle_time));
            });
    }
}