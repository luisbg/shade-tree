use crate::camera::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec::Vec3f;
use crate::visible::HitRecord;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3f },
    Metal { albedo: Vec3f },
}

fn reflect(v: Vec3f, n: Vec3f) -> Vec3f {
    v - (n * v.dot(&n) * 2.0)
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
                let target = rec.p + rec.normal + random_in_unit_sphere();
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
