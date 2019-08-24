use crate::ray::Ray;
use crate::vec::Vec3f;
use crate::visible::Visible;

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3f,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}

impl Visible for Sphere {
    fn hit(&self, ray: Ray) -> Option<f64> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * oc.dot(&ray.direction());
        let c = oc.dot(&oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant >= 0.0 {
            Some((b - discriminant.sqrt()) / (2.0 * a))
        } else {
            None
        }
    }
}
