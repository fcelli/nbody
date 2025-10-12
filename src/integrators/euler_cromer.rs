use super::Integrator;
use crate::system::SystemState;

pub struct EulerCromerIntegrator;

/// Euler-Cromer integrator
///
/// v_{i + 1} = v_i + a_i * dt
///
/// x_{i + 1} = x_i + v_{i + 1} * dt
impl Integrator for EulerCromerIntegrator {
    fn step(&self, state: &mut SystemState, dt: f64) {
        let accelerations = state.compute_accelerations();

        state
            .positions
            .iter_mut()
            .zip(&mut state.velocities)
            .zip(&accelerations)
            .for_each(|((pos, vel), acc)| {
                *vel += *acc * dt;
                *pos += *vel * dt;
            });
    }
}
