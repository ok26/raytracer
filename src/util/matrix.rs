use super::vector::Vec3;

#[derive(Copy, Clone)]
pub struct Mat3 {
    elements: [f64; 9]
}

impl Mat3 {
    pub fn new(elements: [f64; 9]) -> Mat3 {
        Mat3 { elements }
    }

    pub fn zero() -> Mat3 {
        Mat3 { elements: [0.0; 9] }
    }

    pub fn set(&mut self, y: usize, x: usize, value: f64) {
        self.elements[y * 3 + x] = value;
    }

    pub fn get(&self, y: usize, x: usize) -> f64 {
        self.elements[y * 3 + x]
    }

    pub fn rot_matrix(degrees: f64, axis: usize) -> Mat3 {
        match axis {
            0 => Self::rot_x(degrees),
            1 => Self::rot_y(degrees),
            2 => Self::rot_z(degrees),
            _ => panic!("Invalid axis")
        }
    }

    pub fn rot_x(degrees: f64) -> Mat3 {
        let ref pi = std::f64::consts::PI;
        let mut matrix = Mat3::zero();
        matrix.set(0, 0, 1.0);
        matrix.set(1, 1, f64::cos(degrees * pi / 180.0));
        matrix.set(2, 1, f64::sin(degrees * pi / 180.0));
        matrix.set(1, 2, -f64::sin(degrees * pi / 180.0));
        matrix.set(2, 2, f64::cos(degrees * pi / 180.0));
        matrix
    }

    pub fn rot_y(degrees: f64) -> Mat3 {
        let pi = std::f64::consts::PI;
        let mut matrix = Mat3::zero();
        matrix.set(1, 1, 1.0);
        matrix.set(0, 0, f64::cos(degrees * pi / 180.0));
        matrix.set(2, 0, -f64::sin(degrees * pi / 180.0));
        matrix.set(0, 2, f64::sin(degrees * pi / 180.0));
        matrix.set(2, 2, f64::cos(degrees * pi / 180.0));
        matrix
    }

    pub fn rot_z(degrees: f64) -> Mat3 {
        let pi = std::f64::consts::PI;
        let mut matrix = Mat3::zero();
        matrix.set(2, 2, 1.0);
        matrix.set(0, 0, f64::cos(degrees * pi / 180.0));
        matrix.set(1, 0, f64::sin(degrees * pi / 180.0));
        matrix.set(0, 1, -f64::sin(degrees * pi / 180.0));
        matrix.set(1, 1, f64::cos(degrees * pi / 180.0));
        matrix
    }
}

impl std::ops::Add<Mat3> for Mat3 {
    type Output = Mat3;

    fn add(self, rhs: Mat3) -> Mat3 {
        let mut out = Mat3::zero();
        for y in 0..3 {
            for x in 0..3 {
                out.set(y, x, self.get(y, x) + rhs.get(y, x));
            }
        }
        out
    }
}

impl std::ops::Sub<Mat3> for Mat3 {
    type Output = Mat3;

    fn sub(self, rhs: Mat3) -> Mat3 {
        let mut out = Mat3::zero();
        for y in 0..3 {
            for x in 0..3 {
                out.set(y, x, self.get(y, x) - rhs.get(y, x));
            }
        }
        out
    }
}

impl std::ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let mut out = Vec3::zero();
        for y in 0..3 {
            let mut value = 0.0;
            for i in 0..3 {
                value += self.get(y, i) * rhs.get(i);
            }
            out.set(y, value);
        }
        out
    }
}

impl std::fmt::Display for Mat3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}