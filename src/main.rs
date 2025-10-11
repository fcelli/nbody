use nalgebra::Vector2;

use crate::{
    integrators::{EulerIntegrator, Integrator},
    system::SystemState,
};
use std::thread::sleep;
use std::time::Duration;

mod integrators;
mod physics;
mod system;

fn main() {
    let mut state = SystemState::new();
    state.add_body(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), 100.0);
    state.add_body(Vector2::new(1.0, 0.0), Vector2::new(0.0, 10.0), 1.0);

    let integrator = EulerIntegrator;

    loop {
        println!("{},\t{}", state.positions[1][0], state.positions[1][1]);
        integrator.step(&mut state, 0.01);

        sleep(Duration::from_millis(500));
    }
}
