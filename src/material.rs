use crate::ray::Ray;
use crate::vec::Vec3f;
use crate::visible::HitRecord;

#[derive(Default, Clone, Copy)]
pub struct Material {
    pub albedo: Vec3f,
}

fn reflect(v: Vec3f, n: Vec3f) -> Vec3f {
    v - (n * v.dot(&n) * 2.0)
}

impl Material {
    pub fn metal(albedo: Vec3f) -> Material {
        Material { albedo }
    }

    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3f,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(r_in.direction().make_unit_vector(), rec.normal);
        *scattered = Ray::new_from_vec(rec.p, reflected);
        *attenuation = self.albedo;

        scattered.direction().dot(&rec.normal) > 0.0
    }
}
