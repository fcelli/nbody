mod euler;
mod euler_cromer;
mod leapfrog;
mod rk4;

use crate::system::SystemState;
pub use euler::EulerIntegrator;
pub use euler_cromer::EulerCromerIntegrator;
pub use leapfrog::LeapfrogIntegrator;
pub use rk4::RK4Integrator;

pub trait Integrator {
    fn step(&self, state: &mut SystemState, dt: f64);
}
