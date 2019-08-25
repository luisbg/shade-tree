use crate::ray::Ray;
use crate::vec::Vec3f;

#[derive(Default)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3f,
    pub normal: Vec3f,
}

pub trait Visible: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
