use crate::ray::Ray;

pub trait Visible: Send + Sync {
    fn hit(&self, r: Ray) -> Option<f64>;
}
