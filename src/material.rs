use crate::ray::Ray;
use crate::vec::Vec3f;
use crate::visible::HitRecord;
use crate::camera::random_in_unit_sphere;

#[derive(Copy, Clone)]
pub enum Surface {
    Lambian(Material),
    Metal(Material),
}

#[derive(Default, Clone, Copy)]
pub struct Material {
    pub albedo: Vec3f,
}

fn reflect(v: Vec3f, n: Vec3f) -> Vec3f {
    v - (n * v.dot(&n) * 2.0)
}

impl Material {
    pub fn new(albedo: Vec3f) -> Material {
        Material {
            albedo,
        }
    }
}

impl Surface {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3f,
        scattered: &mut Ray,
    ) -> bool {
        match *self {
            Surface::Metal(s) => {
                let reflected = reflect(r_in.direction().make_unit_vector(), rec.normal);
                *scattered = Ray::new_from_vec(rec.p, reflected);
                *attenuation = s.albedo;

                scattered.direction().dot(&rec.normal) > 0.0
            },
            Surface::Lambian(s) => {
                let target = rec.p + rec.normal + random_in_unit_sphere();
                *scattered = Ray::new_from_vec(rec.p, target - rec.p);
                *attenuation = s.albedo;

                true
            }
        }
    }
}

impl Default for Surface {
    fn default() -> Surface {
        Surface::Metal(Material::default())
    }
}
