use crate::ray::Ray;
use crate::vec::Vec3f;
use crate::visible::{HitRecord, Visible};
use crate::world::World;

pub struct Camera {
    origin: Vec3f,
    horizontal: Vec3f,
    vertical: Vec3f,
    lower_left_corner: Vec3f,
}

impl Camera {
    pub fn new(
        origin: Vec3f,
        horizontal: Vec3f,
        vertical: Vec3f,
        lower_left_corner: Vec3f,
    ) -> Camera {
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new_from_vec(
            self.origin,
            self.lower_left_corner + (self.horizontal * u) + (self.vertical * v),
        )
    }
}

pub fn color(r: Ray, vis_obj: &World) -> Vec3f {
    let mut rec = HitRecord::default();

    if vis_obj.hit(r, 0.0, 3.0, &mut rec) {
        let n = rec.normal;
        let color = Vec3f::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
        return color * 0.5;
    }

    let unit_direction = r.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    // Linear interpolation: blended value = ((1 - t) * start_value) + (t * end_value)
    Vec3f::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3f::new(0.1, 0.5, 1.0) * t
}
