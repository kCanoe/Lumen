use super::*;

pub fn lerp(r: &Ray, color_from: Vec3, color_to: Vec3) -> Vec3 {
    let unit_direction = Vec3::unit_vector(r.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * color_from + a * color_to;
}
