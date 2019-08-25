use crate::ray::Ray;
use crate::vec::Vec3f;
use crate::visible::{HitRecord, Visible};

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3f,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Visible for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * oc.dot(&ray.direction());
        let c = oc.dot(&oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at(temp);
                rec.normal = (rec.p - Vec3f::new(0.0, 0.0, -1.0)).make_unit_vector();

                return true;
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at(temp);
                rec.normal = (rec.p - Vec3f::new(0.0, 0.0, -1.0)).make_unit_vector();

                return true;
            }
        }

        false
    }
}
