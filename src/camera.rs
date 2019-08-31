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
    lens_radius: f64,
    u: Vec3f,
    v: Vec3f,
}

impl Camera {
    pub fn new(
        origin: Vec3f,
        lookat: Vec3f,
        vup: Vec3f,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (origin - lookat).make_unit_vector();
        let u = vup.cross(&w).make_unit_vector();
        let v = w.cross(&u);

        let lower_left_corner = origin
            - (u * half_width * focus_dist)
            - (v * half_height * focus_dist)
            - w * focus_dist;
        let horizontal = u * focus_dist * half_width * 2.0;
        let vertical = v * focus_dist * half_height * 2.0;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
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

fn random_in_unit_disk() -> Vec3f {
    let mut rng = rand::thread_rng();
    let mut p: Vec3f;

    while {
        let rnd_x = rng.gen_range(0.0, 1.0);
        let rnd_y = rng.gen_range(0.0, 1.0);
        p = Vec3f::new(rnd_x, rnd_y, 0.0) * 2.0;

        p.dot(&p) >= 1.0
    } {}

    p
}
