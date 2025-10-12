use super::Integrator;
use crate::system::SystemState;

/// Euler integrator
/// x_{i + 1} = x_i + v_i * dt
/// v_{i + 1} = v_i + a_i * dt
pub struct EulerIntegrator;

impl Integrator for EulerIntegrator {
    fn step(&self, state: &mut SystemState, dt: f64) {
        let accelerations = state.compute_accelerations();

        state
            .positions
            .iter_mut()
            .zip(&mut state.velocities)
            .zip(&accelerations)
            .for_each(|((pos, vel), acc)| {
                *pos += *vel * dt;
                *vel += *acc * dt;
            });
    }
}
