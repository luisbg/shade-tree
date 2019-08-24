use crate::ray::Ray;
use crate::vec::Vec3f;

fn hit_sphere(center: Vec3f, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - (radius * radius);
    let discriminant = (b * b) - (4.0 * a * c);

    if discriminant >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

pub fn color(r: Ray) -> Vec3f {
    let t = hit_sphere(Vec3f::new(0.0, 0.0, 1.0), 0.5, r);
    if t > 0.0 {
        return Vec3f::new(1.0, 0.5, 0.0);
    }

    let unit_direction = r.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    // Linear interpolation: blended value = ((1 - t) * start_value) + (t * end_value)
    Vec3f::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3f::new(0.1, 0.5, 1.0) * t
}
