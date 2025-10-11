use crate::system::SystemState;

pub trait Integrator {
    fn step(&self, system: &mut SystemState, dt: f64);
}

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
