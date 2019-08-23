use std::ops::{Add, Mul};

#[derive(Copy, Clone)]
pub struct Vec3f {
    e: [f64; 3],
}

impl Vec3f {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3f {
        Vec3f { e: [x, y, z] }
    }

    #[allow(dead_code)]
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    #[allow(dead_code)]
    pub fn y(&self) -> f64 {
        self.e[1]
    }

    #[allow(dead_code)]
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
            e: [
                self.x() * k,
                self.y() * k,
                self.z() * k,
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
        let r = (o.r() * 255.0) as u32;
        let g = (o.g() * 255.0) as u32;
        let b = (o.b() * 255.0) as u32;

        Vec3i { e: [r, g, b] }
    }

    pub fn to_hex(&self) -> u32 {
        let r = self.e[0] << 16;
        let g = self.e[1] << 8;
        let b = self.e[2];

        r + g + b
    }
}
