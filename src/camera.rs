use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec::Vec3f;
use crate::visible::Visible;

pub fn color(r: Ray, vis_obj: Sphere) -> Vec3f {
    if let Some(t) = vis_obj.hit(r) {
        let normal = (r.point_at(t) - Vec3f::new(0.0, 0.0, -1.0)).make_unit_vector();
        let color = Vec3f::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
        return color * 0.5;
    }

    let unit_direction = r.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    // Linear interpolation: blended value = ((1 - t) * start_value) + (t * end_value)
    Vec3f::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3f::new(0.1, 0.5, 1.0) * t
}
