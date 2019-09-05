use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Default)]
pub struct Vec3f {
    e: [f64; 3],
}

impl Vec3f {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3f {
        Vec3f { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn r(&self) -> f64 {
        self.e[0]
    }

    pub fn g(&self) -> f64 {
        self.e[1]
    }

    pub fn b(&self) -> f64 {
        self.e[2]
    }

    pub fn set_r(&mut self, r: f64) {
        self.e[0] = r;
    }

    pub fn set_g(&mut self, g: f64) {
        self.e[1] = g;
    }

    pub fn set_b(&mut self, b: f64) {
        self.e[2] = b;
    }

    pub fn set_x(&mut self, x: f64) {
        self.e[0] = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.e[1] = y;
    }

    pub fn set_z(&mut self, z: f64) {
        self.e[2] = z;
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn make_unit_vector(&self) -> Vec3f {
        *self / self.length()
    }

    pub fn dot(&self, other: &Vec3f) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn normalize(&mut self) {
        let k = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn squared_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn cross(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            e: [
                self.y() * other.z() - self.z() * other.y(), // a2b3 - a3b2
                self.z() * other.x() - self.x() * other.z(), // a3b1 - a1b3
                self.x() * other.y() - self.y() * other.x(), // a1b2 - a2b1
            ],
        }
    }
}

impl Add for Vec3f {
    type Output = Vec3f;
    fn add(self, other: Vec3f) -> Vec3f {
        Vec3f {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl Mul for Vec3f {
    type Output = Vec3f;
    fn mul(self, k: Vec3f) -> Vec3f {
        Vec3f {
            e: [self.x() * k.x(), self.y() * k.y(), self.z() * k.z()],
        }
    }
}

impl Mul<f64> for Vec3f {
    type Output = Vec3f;
    fn mul(self, k: f64) -> Vec3f {
        Vec3f {
            e: [self.x() * k, self.y() * k, self.z() * k],
        }
    }
}

impl Div for Vec3f {
    type Output = Vec3f;
    fn div(self, other: Vec3f) -> Vec3f {
        Vec3f {
            e: [
                self.x() / other.x(),
                self.y() / other.y(),
                self.z() / other.z(),
            ],
        }
    }
}

impl Div<f64> for Vec3f {
    type Output = Vec3f;
    fn div(self, k: f64) -> Vec3f {
        Vec3f {
            e: [self.x() / k, self.y() / k, self.z() / k],
        }
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;
    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f {
            e: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl PartialEq for Vec3f {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl fmt::Debug for Vec3f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vec3f {{ x: {}, y: {}, z: {} }}",
            self.x(),
            self.y(),
            self.z()
        )
    }
}

pub struct Vec3i {
    e: [u32; 3],
}

impl Vec3i {
    #[allow(dead_code)]
    pub fn new(x: u32, y: u32, z: u32) -> Vec3i {
        Vec3i { e: [x, y, z] }
    }

    pub fn new_from_f64(o: Vec3f) -> Vec3i {
        let r = if o.r() >= 0.0 {
            (o.r() * 255.0) as u32
        } else {
            0
        };
        let g = if o.g() >= 0.0 {
            (o.g() * 255.0) as u32
        } else {
            0
        };
        let b = if o.b() >= 0.0 {
            (o.b() * 255.0) as u32
        } else {
            0
        };

        Vec3i { e: [r, g, b] }
    }

    pub fn to_hex(&self) -> u32 {
        let r = self.e[0] << 16;
        let g = self.e[1] << 8;
        let b = self.e[2];

        r + g + b
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::vec::Vec3f;

    #[test]
    fn vec3f_default_and_new() {
        let def = Vec3f::default();
        assert_eq!(def.x(), 0.0);
        assert_eq!(def.y(), 0.0);
        assert_eq!(def.z(), 0.0);

        let new = Vec3f::new(1.0, 2.0, 3.0);
        assert_eq!(new.x(), 1.0);
        assert_eq!(new.y(), 2.0);
        assert_eq!(new.z(), 3.0);
    }

    #[test]
    fn vec3f_set_and_get() {
        let mut t = Vec3f::default();
        t.set_r(1.0);
        t.set_g(2.0);
        t.set_b(3.0);

        assert_eq!(t.r(), 1.0);
        assert_eq!(t.g(), 2.0);
        assert_eq!(t.b(), 3.0);

        t.set_x(4.0);
        t.set_y(5.0);
        t.set_z(6.0);

        assert_eq!(t.x(), 4.0);
        assert_eq!(t.y(), 5.0);
        assert_eq!(t.z(), 6.0);
    }

    #[test]
    fn vec3f_length() {
        let t = Vec3f::new(1.0, 2.0, 3.0);
        assert_eq!(t.length(), 3.7416573867739413);
    }

    #[test]
    fn vec3f_make_unit_vector() {
        let t = Vec3f::new(1.0, 2.0, 3.0);
        assert_eq!(
            t.make_unit_vector(),
            Vec3f::new(0.2672612419124244, 0.5345224838248488, 0.8017837257372732)
        );
    }
}
