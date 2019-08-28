use crate::vec::Vec3f;

#[derive(Copy, Clone, Default)]
pub struct Ray {
    orig: Vec3f,
    dir: Vec3f,
}

impl Ray {
    pub fn new(orig: Vec3f, dir: Vec3f) -> Ray {
        let mut dir = dir;
        dir.normalize();
        Ray { orig, dir }
    }

    #[allow(dead_code)]
    pub fn point_at_parameter(&self, t: f64) -> Vec3f {
        self.orig + (self.dir * t)
    }

    pub fn origin(&self) -> Vec3f {
        self.orig
    }

    pub fn direction(&self) -> Vec3f {
        self.dir
    }

    pub fn point_at(&self, t: f64) -> Vec3f {
        self.orig + self.dir * t
    }
}
