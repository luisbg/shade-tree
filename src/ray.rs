use crate::vec::Vec3f;

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
}
