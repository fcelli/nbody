use nalgebra::Vector2;
use nbody::{Simulation, integrators::IntegratorType};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum JsIntegratorType {
    Euler,
    EulerCromer,
    RK4,
    Leapfrog,
}

impl From<JsIntegratorType> for IntegratorType {
    fn from(js: JsIntegratorType) -> Self {
        match js {
            JsIntegratorType::Euler => Self::Euler,
            JsIntegratorType::EulerCromer => Self::EulerCromer,
            JsIntegratorType::RK4 => Self::RK4,
            JsIntegratorType::Leapfrog => Self::Leapfrog,
        }
    }
}

#[wasm_bindgen]
pub struct WasmSim {
    sim: Simulation,
}

#[wasm_bindgen]
impl WasmSim {
    #[wasm_bindgen(constructor)]
    pub fn new(integrator_type: JsIntegratorType) -> Self {
        let mut sim = Simulation::new(integrator_type.into());

        sim.state
            .add_body(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), 100.0);
        sim.state
            .add_body(Vector2::new(1.0, 0.0), Vector2::new(0.0, 10.0), 1.0);
        sim.state
            .add_body(Vector2::new(-0.5, 0.0), Vector2::new(0.0, -14.0), 1.0);
        sim.state
            .add_body(Vector2::new(0.0, 3.0), Vector2::new(-4.0, 0.0), 1.0);

        Self { sim }
    }

    pub fn step(&mut self, dt: f64) {
        self.sim.step(dt);
    }

    pub fn get_positions(&self) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for pos in &self.sim.state.positions {
            let pair = js_sys::Array::new();
            pair.push(&JsValue::from_f64(pos.x));
            pair.push(&JsValue::from_f64(pos.y));
            arr.push(&pair);
        }
        arr
    }
}
