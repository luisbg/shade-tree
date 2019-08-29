use crate::ray::Ray;
use crate::vec::Vec3f;
use crate::visible::{HitRecord, Visible};
use crate::world::World;

#[derive(Copy, Clone)]
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
        Ray::new(
            self.origin,
            self.lower_left_corner + (self.horizontal * u) + (self.vertical * v),
        )
    }
}

pub fn color(r: Ray, vis_obj: &World, depth: usize) -> Vec3f {
    let mut rec = HitRecord::default();

    if vis_obj.hit(r, 0.0001, std::f64::MAX, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3f::default();
        let rec_cp = HitRecord {
            p: rec.p,
            normal: rec.normal,
            t: rec.t,
            material: rec.material,
        };

        if depth < 50
            && rec
                .material
                .scatter(&r, &rec_cp, &mut attenuation, &mut scattered)
        {
            return attenuation * color(scattered, vis_obj, depth + 1);
        } else {
            return Vec3f::default();
        }
    }

    let t = 0.5 * (r.direction().y() + 1.0);

    // Linear interpolation: blended value = ((1 - t) * start_value) + (t * end_value)
    Vec3f::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3f::new(0.5, 0.7, 1.0) * t
}
