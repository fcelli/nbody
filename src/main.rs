use nalgebra::Vector2;

use crate::system::NBodySystem;

mod physics;
mod system;

fn main() {
    let mut nbody_system = NBodySystem::new();
    nbody_system.add_body(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), 1.0);
    nbody_system.add_body(Vector2::new(1.0, 0.0), Vector2::new(0.0, 0.0), 1.0);

    let accelerations = nbody_system.compute_accelerations();

    println!("Number of bodies in the system: {}", nbody_system.len());
    accelerations.iter().for_each(|x| println!("{}", x));
}
