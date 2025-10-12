use nalgebra::Vector2;

use super::Integrator;
use crate::system::SystemState;

/// Runge-Kutta integrator
///
/// x_{i + 1} = x_i + (1 / 6) * (k1 + 2 * k2 + 2 * k3 + k4)
///
/// v_{i + 1} = v_i + (1 / 6) * (k1 + 2 * k2 + 2 * k3 + k4)
pub struct RK4Integrator;

impl RK4Integrator {
    fn derive(&self, state: &SystemState) -> K {
        let accelerations = state.compute_accelerations();

        K::new(state.velocities.clone(), accelerations)
    }

    fn update(
        &self,
        state: &mut SystemState,
        init_pos: &[Vector2<f64>],
        init_vel: &[Vector2<f64>],
        k: &K,
        dt: f64,
    ) {
        for i in 0..state.len() {
            state.positions[i] = init_pos[i] + k.positions[i] * dt;
            state.velocities[i] = init_vel[i] + k.velocities[i] * dt;
        }
    }
}

impl Integrator for RK4Integrator {
    fn step(&self, state: &mut SystemState, dt: f64) {
        let initial_positions = state.positions.clone();
        let initial_velocities = state.velocities.clone();

        let dt_div2 = dt / 2.0;
        let dt_div6 = dt / 6.0;

        let k1 = self.derive(state);
        self.update(state, &initial_positions, &initial_velocities, &k1, dt_div2);

        let k2 = self.derive(state);
        self.update(state, &initial_positions, &initial_velocities, &k1, dt_div2);

        let k3 = self.derive(state);
        self.update(state, &initial_positions, &initial_velocities, &k1, dt);

        let k4 = self.derive(state);
        for i in 0..state.len() {
            state.positions[i] = initial_positions[i]
                + (k1.positions[i]
                    + k2.positions[i] * 2.0
                    + k3.positions[i] * 2.0
                    + k4.positions[i])
                    * dt_div6;

            state.velocities[i] = initial_velocities[i]
                + (k1.velocities[i]
                    + k2.velocities[i] * 2.0
                    + k3.velocities[i] * 2.0
                    + k4.velocities[i])
                    * dt_div6;
        }
    }
}

struct K {
    positions: Vec<Vector2<f64>>,
    velocities: Vec<Vector2<f64>>,
}

impl K {
    pub fn new(positions: Vec<Vector2<f64>>, velocities: Vec<Vector2<f64>>) -> Self {
        Self {
            positions: positions,
            velocities: velocities,
        }
    }
}
