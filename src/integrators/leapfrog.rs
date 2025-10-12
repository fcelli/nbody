use super::Integrator;
use crate::system::SystemState;

/// Leapfrog integrator
///
/// v_{i + 1 / 2} = v_i + a_i * dt / 2
///
/// x_{i + 1} = x_i + v_{i + 1 / 2} * dt
///
/// v_{i + 1} = v_{i + 1 / 2} + a_{i + 1} * dt / 2
pub struct LeapfrogIntegrator;

impl Integrator for LeapfrogIntegrator {
    fn step(&self, state: &mut SystemState, dt: f64) {
        let dt_div2 = dt / 2.0;
        let accelerations = state.compute_accelerations();

        state
            .positions
            .iter_mut()
            .zip(&mut state.velocities)
            .zip(&accelerations)
            .for_each(|((pos, vel), acc)| {
                *vel += acc * dt_div2;
                *pos += *vel * dt;
            });

        let accelerations = state.compute_accelerations();

        state
            .velocities
            .iter_mut()
            .zip(&accelerations)
            .for_each(|(vel, acc)| {
                *vel += acc * dt_div2;
            });
    }
}
