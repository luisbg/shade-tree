use crate::ray::Ray;
use crate::vec::Vec3f;

pub fn color(r: Ray) -> Vec3f {
    let unit_direction = r.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    // Linear interpolation: blended value = ((1 - t) * start_value) + (t * end_value)
    Vec3f::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3f::new(0.1, 0.5, 1.0) * t
}
