use std::{
    ops::Deref,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

use eframe::egui::Context;
use rusty_roads::Roads;
use wasm_bindgen::prelude::wasm_bindgen;

use geo_types::{Coord, coord};

#[derive(Debug)]
pub struct Transform {
    pub rotate_mat: glam::Mat2,
    pub scale_mat: glam::Mat2,
    pub translate: glam::Vec2,
}

impl Transform {
    pub fn scale_rotate(&self) -> glam::Mat2 {
        self.scale_mat * self.rotate_mat
    }

    pub fn rotate(&mut self, rad: f32) {
        let rotate = glam::Mat2::from_cols(
            glam::Vec2::new(rad.cos(), rad.sin()),
            glam::Vec2::new(-rad.sin(), rad.cos()),
        );

        self.rotate_mat *= rotate;
    }

    pub fn scale(&mut self, factor: f32) {
        self.scale_mat *= factor;
    }

    pub fn translate(&mut self, vec: eframe::egui::Vec2) {
        self.translate += glam::Vec2::new(vec.x, vec.y);
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            rotate_mat: glam::Mat2::IDENTITY,
            scale_mat: glam::Mat2::IDENTITY,
            translate: glam::Vec2::ZERO,
        }
    }
}

#[derive(Debug, Default)]
pub struct State {
    pub user_location: Mutex<Vec<Coord<f64>>>,
    pub location_timestamp: Mutex<Vec<web_time::SystemTime>>,
    pub roads: Mutex<Roads>,
    pub context: Mutex<Context>,
    pub transform: Mutex<Transform>,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Clone)]
#[wasm_bindgen]
pub struct StateHandle(Arc<State>);

#[wasm_bindgen]
impl StateHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
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
