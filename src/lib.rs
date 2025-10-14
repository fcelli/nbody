use crate::{
    integrators::{
        EulerCromerIntegrator, EulerIntegrator, Integrator, IntegratorType, LeapfrogIntegrator,
        RK4Integrator,
    },
    system::SystemState,
};

pub mod integrators;
pub mod physics;
pub mod system;

pub struct Simulation {
    pub state: SystemState,
    pub integrator: Box<dyn Integrator>,
}

impl Simulation {
    pub fn new(integrator_type: IntegratorType) -> Self {
        let state = SystemState::new();

        let integrator: Box<dyn Integrator> = match integrator_type {
            IntegratorType::Euler => Box::new(EulerIntegrator),
            IntegratorType::EulerCromer => Box::new(EulerCromerIntegrator),
            IntegratorType::RK4 => Box::new(RK4Integrator),
            IntegratorType::Leapfrog => Box::new(LeapfrogIntegrator),
        };

        Self { state, integrator }
    }

    pub fn step(&mut self, dt: f64) {
        self.integrator.step(&mut self.state, dt);
    }
}
