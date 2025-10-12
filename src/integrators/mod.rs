mod euler;
mod euler_cromer;

use crate::system::SystemState;
pub use euler::EulerIntegrator;
pub use euler_cromer::EulerCromerIntegrator;

pub trait Integrator {
    fn step(&self, state: &mut SystemState, dt: f64);
}
