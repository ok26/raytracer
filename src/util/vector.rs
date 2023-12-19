#[derive(Copy, Clone)]
pub struct Vec3 {
    elements: [f64; 3]
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        let mut out = Vec3::zero();
        for i in 0..3 {
            out.set(i, self.get(i) + rhs.get(i));
        }
        out
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        let mut out = Vec3::zero();
        for i in 0..3 {
            out.set(i, self.get(i) - rhs.get(i));
        }
        out
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Vec3) -> f64 {
        self.elements.iter().zip(rhs.elements.iter()).map(|(e1, e2)| e1 * e2).sum()
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        let mut out = Vec3::zero();
        for i in 0..3 {
            out.set(i, self.get(i) * rhs);
        }
        out
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}

impl Vec3 {
    pub fn new(elements: [f64; 3]) -> Vec3 {
        Vec3 { elements }
    }

    pub fn zero() -> Vec3 {
        Vec3 {  elements: [0.0; 3] }
    }

    pub fn set(&mut self, pos: usize, value: f64) {
        self.elements[pos] = value;
    }

    pub fn get(&self, pos: usize) -> f64 {
        self.elements[pos]
    }

    pub fn length(&self) -> f64 {
        self.elements.iter().map(|e| e * e).sum::<f64>().sqrt()
    }

    pub fn normalize(&mut self) {
        *self = *self * (1.0 / self.length());
    }

    pub fn normalized(&self) -> Vec3 {
        *self * (1.0 / self.length())
    }

    pub fn cross(&self, vec: Vec3) -> Vec3 {
        let mut out = Vec3::zero();
        out.set(0, self.get(1) * vec.get(2) - self.get(2) * vec.get(1));
        out.set(1, self.get(2) * vec.get(0) - self.get(0) * vec.get(2));
        out.set(2, self.get(0) * vec.get(1) - self.get(1) * vec.get(0));
        out
    }

    pub fn project(&self, vec: Vec3) -> Vec3 {
        vec * (vec * *self * (1.0 / (vec * vec)))
    }

    pub fn reflect(&self, vec: Vec3) -> Vec3 {
        *self - vec * ((*self * vec) * 2.0)
    }
}

pub fn lerp(vec1: Vec3, vec2: Vec3, step: f64) -> Vec3 {
    vec1 + (vec2 - vec1) * step
}