use crate::ray::Ray;
use crate::vec::Vec3f;
use crate::visible::HitRecord;
use rand::Rng;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3f },
    Metal { albedo: Vec3f },
}

fn reflect(v: Vec3f, n: Vec3f) -> Vec3f {
    v - (n * v.dot(&n) * 2.0)
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
            Material::Metal { ref albedo } => {
                let reflected = reflect(r_in.direction(), rec.normal);
                *scattered = Ray::new(rec.p, reflected);
                *attenuation = *albedo;

                scattered.direction().dot(&rec.normal) > 0.0
            }
            Material::Lambertian { ref albedo } => {
                let target = rec.p + rec.normal + random_point_in_unit_sphere();
                *scattered = Ray::new(rec.p, target - rec.p);
                *attenuation = *albedo;

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
