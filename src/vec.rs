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

    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
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
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
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
