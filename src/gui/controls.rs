use std::sync::{Arc, Mutex};
use imgui::{ChildWindow, Slider, Ui, VerticalSlider};
use crate::engine::audio::EngineManager;
use crate::system::parameter::Parameter;
use crate::system::parameter::ParameterID::{KSAmount, WS1Amount, WT1Amount};

use super::WindowContext;

pub struct ControlsWindow;
impl ControlsWindow {
    pub fn build(ui: &Ui, context: WindowContext, state: &mut serde_json::Value) {
        let params = Parameter::supported();
        let fwy = context.height;
        let fwx = context.width;

        ui.window("Voice Controls")
            .size([270.0, 720.0-45.0], imgui::Condition::FirstUseEver)
            .position([1010.0, 45.0], imgui::Condition::FirstUseEver)
            .build(|| {
                for (i, p) in params.iter().enumerate() {
                    let n = format!("{:?}", p);

                    ui.text(&n);
                    let mut value = state["parameters"][&n].as_f64().unwrap();
                    let edited = ui.slider_config(format!("##slider-{}", i), 0.0, 1.0)
                        .build(&mut value);

                    if edited {
                        state["parameters"][&n] = serde_json::Value::Number(serde_json::Number::from_f64(value).unwrap());
                        context.engine.lock().unwrap().set_parameter(*p, value as f32);
                    }
                }
            });
    }
}