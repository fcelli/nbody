use nalgebra::Vector2;

use crate::physics::gravitational_acceleration_unit_mass;

pub struct NBodySystem {
    positions: Vec<Vector2<f64>>,
    velocities: Vec<Vector2<f64>>,
    masses: Vec<f64>,
}

impl NBodySystem {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            velocities: Vec::new(),
            masses: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.positions.len()
    }

    pub fn add_body(&mut self, position: Vector2<f64>, velocity: Vector2<f64>, mass: f64) {
        self.positions.push(position);
        self.velocities.push(velocity);
        self.masses.push(mass);
    }

    pub fn compute_accelerations(&self) -> Vec<Vector2<f64>> {
        let n = self.len();
        let mut accelerations = vec![Vector2::zeros(); n];
        for i in 0..n {
            for j in i + 1..n {
                let unit_mass_acc =
                    gravitational_acceleration_unit_mass(&self.positions[i], &self.positions[j]);
                accelerations[i] += self.masses[j] * unit_mass_acc;
                accelerations[j] -= self.masses[i] * unit_mass_acc;
            }
        }
        accelerations
    }
}
