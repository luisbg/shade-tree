use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3f;
use crate::visible::{HitRecord, Visible};

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3f,
    radius: f64,
    record: HitRecord,
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f64, record: HitRecord) -> Sphere {
        Sphere {
            center,
            radius,
            record,
        }
    }
}

impl Visible for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);

        if discriminant > 0.0 {
            rec.material = self.record.material;

            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at(temp);
                rec.normal = rec.p - self.center;
                rec.normal.normalize();

                return true;
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at(temp);
                rec.normal = rec.p - self.center;
                rec.normal.normalize();

                return true;
            }
        }

        false
    }

    fn set_material(&mut self, material: Material) {
        self.record.material = material;
    }
}
