use eframe::{
    egui::{self, Color32, Pos2, Sense, Shape, Stroke, Vec2},
    epaint::{CircleShape, PathShape, PathStroke},
};
use geo_types::{Coord, coord};
use map::map;
use state::StateHandle;
use wasm_bindgen::prelude::*;

pub mod backend;
pub mod map;
pub mod state;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
struct WebHandle {
    runner: eframe::WebRunner,
    state: state::StateHandle,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebHandle {
    #[wasm_bindgen(constructor)]
    pub fn new(state: state::StateHandle) -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();

        Self {
            runner: eframe::WebRunner::new(),
            state,
        }
    }

    #[wasm_bindgen]
    pub async fn start(
        &self,
        canvas: web_sys::HtmlCanvasElement,
    ) -> Result<(), wasm_bindgen::JsValue> {
        let state = self.state.clone();
        self.runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(move |cc| Ok(Box::new(App::new(cc, state)))),
            )
            .await
    }
}

#[allow(dead_code)]
#[derive(Clone)]
struct App {
    state: state::StateHandle,
}

impl App {
    #[allow(dead_code)]
    fn new(ctx: &eframe::CreationContext<'_>, state: StateHandle) -> Self {
        let mut context = state.context.lock().unwrap();
        *context = ctx.egui_ctx.clone();
        drop(context);
        Self { state }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            map(ui, self.state.clone());
        });
    }
}
