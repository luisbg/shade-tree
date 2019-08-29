use crate::ray::Ray;
use crate::vec::Vec3f;
use crate::visible::HitRecord;
use rand::Rng;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3f },
    Metal { albedo: Vec3f, fuzz: f64 },
    Dielectric { ri: f64 },
}

fn reflect(v: Vec3f, n: Vec3f) -> Vec3f {
    v - (n * v.dot(&n) * 2.0)
}

fn refract(v: Vec3f, n: Vec3f, ni_over_nt: f64) -> Option<Vec3f> {
    let uv = v.make_unit_vector();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(((uv - n * dt) * ni_over_nt) - (n * discriminant.sqrt()))
    } else {
        None
    }
}

fn random_point_in_unit_sphere() -> Vec3f {
    let mut rng = rand::thread_rng();
    let mut p: Vec3f;

    while {
        let rnd_x = rng.gen_range(-1.0, 1.0);
        let rnd_y = rng.gen_range(-1.0, 1.0);
        let rnd_z = rng.gen_range(-1.0, 1.0);
        p = Vec3f::new(rnd_x, rnd_y, rnd_z);

        p.squared_length() < 1.0
    } {}

    p
}

fn shlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3f,
        scattered: &mut Ray,
    ) -> bool {
        match *self {
            Material::Metal {
                ref albedo,
                ref fuzz,
            } => {
                let reflected = reflect(r_in.direction(), rec.normal);
                *scattered = Ray::new(rec.p, reflected + random_point_in_unit_sphere() * (*fuzz));
                *attenuation = *albedo;

                scattered.direction().dot(&rec.normal) > 0.0
            }
            Material::Lambertian { ref albedo } => {
                let target = rec.p + rec.normal + random_point_in_unit_sphere();
                *scattered = Ray::new(rec.p, target - rec.p);
                *attenuation = *albedo;

                true
            }
            Material::Dielectric { ref ri } => {
                let mut rng = rand::thread_rng();
                *attenuation = Vec3f::new(1.0, 1.0, 1.0);

                let reflected = reflect(r_in.direction(), rec.normal);
                let outward_normal: Vec3f;
                let ni_over_nt: f64;
                let cosine: f64;
                let reflect_prob: f64;

                if r_in.direction().dot(&rec.normal) > 0.0 {
                    outward_normal = rec.normal * -1.0;
                    ni_over_nt = *ri;
                    cosine = ri * r_in.direction().dot(&rec.normal) / r_in.direction().length();
                } else {
                    outward_normal = rec.normal;
                    ni_over_nt = 1.0 / *ri;
                    cosine = (r_in.direction().dot(&rec.normal) * -1.0) / r_in.direction().length();
                }

                let refracted = refract(r_in.direction(), outward_normal, ni_over_nt);
                if let Some(_refracted) = refracted {
                    reflect_prob = shlick(cosine, *ri);
                } else {
                    reflect_prob = 1.0;
                }

                if rng.gen_range(0.0, 1.0) < reflect_prob {
                    *scattered = Ray::new(rec.p, reflected);
                } else {
                    *scattered = Ray::new(rec.p, refracted.unwrap());
                }

                true
            }
        }
    }
}

impl Default for Material {
    fn default() -> Material {
        Material::Lambertian {
            albedo: Vec3f::new(0.8, 0.8, 0.8),
        }
    }
}
