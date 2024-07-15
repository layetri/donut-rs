use std::sync::{Arc, Mutex};
use imgui::{ChildWindow, Ui, VerticalSlider};
use crate::engine::audio::EngineManager;
use crate::system::parameter::Parameter;
use crate::system::parameter::ParameterID::{KSAmount, WS1Amount, WT1Amount};

pub struct MixerWindow;
impl MixerWindow {
    pub fn build(ui: &Ui, engine: Arc<Mutex<EngineManager>>, state: &mut serde_json::Value) {
        let params = &[WS1Amount, WT1Amount, KSAmount];

        ui.window("Mixer")
            .size([800.0, 240.0], imgui::Condition::FirstUseEver)
            .build(|| {
                for (i, p) in params.iter().enumerate() {
                    let n = format!("{:?}", p);

                    ui.same_line();
                    let token = ui.begin_group();
                    ui.child_window(&n).size([80.0, 200.0]).build(|| {
                        let window_width = ui.window_size()[0];
                        let text_width = ui.calc_text_size(&n)[0];
                        ui.set_cursor_pos([(window_width - text_width) / 2.0, 0.0]);
                        ui.text(&n);

                        ui.set_cursor_pos([(window_width - 18.0) * 0.5, 20.0]);
                        let mut value = state["parameters"][&n].as_f64().unwrap();
                        let edited = VerticalSlider::new("##mixer_slider", [18.0, 160.0], 0.0, 1.0)
                            .build(ui, &mut value);

                        if edited {
                            state["parameters"][&n] = serde_json::Value::Number(serde_json::Number::from_f64(value).unwrap());
                            engine.lock().unwrap().set_parameter(*p, value as f32);
                        }
                    });
                    token.end();
                }
            });
    }
}