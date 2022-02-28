pub(crate) struct Vector3(pub(crate) f64, pub(crate) f64, pub(crate) f64);

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