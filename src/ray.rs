use crate::vec::Vec3f;

#[derive(Copy, Clone)]
pub struct Ray {
    orig: Vec3f,
    dir: Vec3f,
}


impl Ray {
    #[allow(dead_code)]
    pub fn new() -> Ray {
        Ray {
            orig: Vec3f::new(0.0, 0.0, 0.0),
            dir: Vec3f::new(1.0, 1.0, 1.0)
        }
    }

    #[allow(dead_code)]
    pub fn new_from_vec(o: Vec3f, d: Vec3f) -> Ray {
        Ray { orig: o,
             dir: d
            }
    }

    #[allow(dead_code)]
    pub fn point_at_parameter(&self, t: f64) -> Vec3f {
        self.orig + (self.dir * t)
    }

    #[allow(dead_code)]
    pub fn origin(&self) -> Vec3f {
        self.orig
    }

    #[allow(dead_code)]
    pub fn direction(&self) -> Vec3f {
        self.dir
    }
}
