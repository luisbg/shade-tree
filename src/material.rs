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
                *attenuation = Vec3f::new(1.0, 1.0, 1.0);

                let reflected = reflect(r_in.direction(), rec.normal);
                let mut outward_normal = Vec3f::default();
                let ni_over_nt: f64;

                if r_in.direction().dot(&rec.normal) > 0.0 {
                    outward_normal = outward_normal - rec.normal;
                    ni_over_nt = *ri;
                } else {
                    outward_normal = rec.normal;
                    ni_over_nt = 1.0 / *ri;
                }

                match refract(r_in.direction(), outward_normal, ni_over_nt) {
                    Some(refracted) => {
                        *scattered = Ray::new(rec.p, refracted);
                    }
                    None => {
                        *scattered = Ray::new(rec.p, reflected);
                        return false;
                    }
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
