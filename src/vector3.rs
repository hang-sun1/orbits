use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Vector3(pub(crate) f64, pub(crate) f64, pub(crate) f64);

impl Vector3 {
    pub(crate) fn len(&self) -> f64 {
        (self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt()
    }

    pub(crate) fn unit(&self) -> Self {
        let n = self.len();
        Vector3(self.0 / n, self.1 / n, self.2 /n)
    }
}

impl From<(f64, f64, f64)> for Vector3 {
    fn from(x: (f64, f64, f64)) -> Self {
        Self(x.0, x.1, x.2) 
    }
}

impl Into<(f64, f64, f64)> for Vector3 {
    fn into(self) -> (f64, f64, f64) {
        (self.0, self.1, self.2)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0+rhs.0, self.1+rhs.1, self.2+rhs.2)
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0-rhs.0, self.1-rhs.1, self.2-rhs.2)
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;
    
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3(self*rhs.0, self*rhs.1, self*rhs.2)
    }
}