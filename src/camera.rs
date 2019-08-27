use crate::ray::Ray;
use crate::vec::Vec3f;
use crate::visible::{HitRecord, Visible};
use crate::world::World;
use rand::Rng;

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
        Ray::new_from_vec(
            self.origin,
            self.lower_left_corner + (self.horizontal * u) + (self.vertical * v),
        )
    }
}

#[allow(dead_code)]
fn random_in_unit_sphere() -> Vec3f {
    let mut rng = rand::thread_rng();
    let mut p: Vec3f;

    while {
        let rnd_x = rng.gen_range(-1.0, 1.0);
        let rnd_y = rng.gen_range(-1.0, 1.0);
        let rnd_z = rng.gen_range(-1.0, 1.0);
        p = Vec3f::new(rnd_x, rnd_y, rnd_z) - Vec3f::new(-1.0, -1.0, -1.0);

        p.squared_length() < 1.0
    } {}

    p
}

pub fn color(r: Ray, vis_obj: &World, depth: usize) -> Vec3f {
    let mut rec = HitRecord::default();

    if vis_obj.hit(r, 0.0, 100.0, &mut rec) {
        let mut scattered = Ray::new();
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

    let unit_direction = r.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    // Linear interpolation: blended value = ((1 - t) * start_value) + (t * end_value)
    Vec3f::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3f::new(0.1, 0.5, 1.0) * t
}
