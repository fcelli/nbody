use nalgebra::Vector2;

const G: f64 = 1.0;

/// Computes the gravitational acceleration at position `pos1` due to the presence
/// of a unit mass body at position `pos2`.
pub fn gravitational_acceleration_unit_mass(
    pos1: &Vector2<f64>,
    pos2: &Vector2<f64>,
) -> Vector2<f64> {
    let direction = pos2 - pos1;
    let r: f64 = direction.norm();
    if r == 0.0 {
        // If the bodies overlap, do not apply any force.
        return Vector2::zeros();
    }
    let magnitude: f64 = G / (r * r);
    direction.normalize() * magnitude
}
