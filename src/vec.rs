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

    #[allow(dead_code)]
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    #[allow(dead_code)]
    pub fn r(&self) -> f64 {
        self.e[0]
    }

    #[allow(dead_code)]
    pub fn g(&self) -> f64 {
        self.e[1]
    }

    #[allow(dead_code)]
    pub fn b(&self) -> f64 {
        self.e[2]
    }
}
