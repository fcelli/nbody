use nalgebra::Vector2;
use nbody::{
    integrators::{Integrator, LeapfrogIntegrator},
    system::SystemState,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmSim {
    state: SystemState,
    integrator: LeapfrogIntegrator,
}

#[wasm_bindgen]
impl WasmSim {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut state = SystemState::new();
        state.add_body(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), 100.0);
        state.add_body(Vector2::new(1.0, 0.0), Vector2::new(0.0, 10.0), 1.0);
        state.add_body(Vector2::new(-0.5, 0.0), Vector2::new(0.0, -14.0), 1.0);
        state.add_body(Vector2::new(0.0, 3.0), Vector2::new(-4.0, 0.0), 1.0);
        Self {
            state,
            integrator: LeapfrogIntegrator,
        }
    }

    pub fn step(&mut self, dt: f64) {
        self.integrator.step(&mut self.state, dt);
    }

    pub fn get_positions(&self) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for pos in &self.state.positions {
            let pair = js_sys::Array::new();
            pair.push(&JsValue::from_f64(pos.x));
            pair.push(&JsValue::from_f64(pos.y));
            arr.push(&pair);
        }
        arr
    }
}
