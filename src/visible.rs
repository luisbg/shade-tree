use crate::material::Surface;
use crate::ray::Ray;
use crate::vec::Vec3f;

#[derive(Default, Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3f,
    pub normal: Vec3f,
    pub surface: Surface,
}

pub trait Visible: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, surface: &mut HitRecord) -> bool;
    fn set_surface(&mut self, surface: Surface);
}
