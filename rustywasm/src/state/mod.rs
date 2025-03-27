pub mod transform;
pub use transform::*;

use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use eframe::egui::Context;
use geo_types::{Coord, LineString, coord};
use rusty_roads::Roads;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::PrintDebug;

#[derive(Debug, Default)]
pub struct State {
    pub user_location: Mutex<Vec<Coord<f64>>>,
    pub location_timestamp: Mutex<Vec<web_time::SystemTime>>,
    pub roads: Mutex<Roads>,
    pub road_index: Mutex<rusty_roads::RoadIndex>,
    pub context: Mutex<Context>,
    pub transform: Mutex<Transform>,
}

impl State {
    pub fn new() -> Self {
        let slf = Self::default();
        let road = LineString::from_iter(vec![
            coord! {x:9.990876, y:57.011908},
            coord! {x:9.991820, y:57.012704},
            coord! {x:9.991776, y:57.011749},
        ]);
        slf.road_index
            .lock()
            .print_debug("Road Index")
            .unwrap()
            .insert(0, road);
        slf
    }
}

#[derive(Default, Clone)]
#[wasm_bindgen]
pub struct StateHandle(Arc<State>);

#[wasm_bindgen]
impl StateHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Arc::new(State::new()))
    }

    #[wasm_bindgen]
    pub fn clone(&self) -> Self {
        Clone::clone(self)
    }

    pub fn add_point(&self, lat: f64, lon: f64) {
        let time = web_time::SystemTime::now();
        let mut user_location = self.user_location.lock().unwrap();
        user_location.push(coord! {x:lat, y:lon});
        drop(user_location);
        let mut location_timestamp = self.location_timestamp.lock().unwrap();
        location_timestamp.push(time);
        drop(location_timestamp);
        let context = self.context.lock().unwrap();
        context.request_repaint();
    }
}

impl Deref for StateHandle {
    type Target = State;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
