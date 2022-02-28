use crate::{SolarObject, JulianDate, vector3::Vector3};
use vsop87::vsop87a;

pub(crate) struct Mercury {
    pub(crate) time: JulianDate,
}

pub(crate) struct Venus {
    pub(crate) time: JulianDate,
}

pub(crate) struct Earth {
    pub(crate) time: JulianDate,
}

pub(crate) struct Mars {
    pub(crate) time: JulianDate,
}

pub(crate) struct Saturn {
    pub(crate) time: JulianDate,
}

pub(crate) struct Jupiter {
    pub(crate) time: JulianDate,
}

pub(crate) struct Uranus {
    pub(crate) time: JulianDate,
}

pub(crate) struct Neptune {
    pub(crate) time: JulianDate,
}

impl SolarObject for Mercury {
    fn coords(&self) -> Vector3 {
        let coords = vsop87a::mercury(self.time.0);
        Vector3::from((coords.x, coords.y, coords.z))
    }

    fn tick(&mut self, delta_t: f64) {
        self.time.0 += delta_t;
    }

    fn name(&self) -> String {
        "Mercury".to_string()
    }

    fn mass(&self) -> f64 {
        3.285e23 
    }
}

impl SolarObject for Venus {
    fn coords(&self)-> Vector3 {
        let coords = vsop87a::venus(self.time.0);
        Vector3::from((coords.x, coords.y, coords.z))
    }
    
    fn tick(&mut self, delta_t: f64) {
        self.time.0 += delta_t;
    }

    fn name(&self) -> String {
        "Venus".to_string()
    }
    
    fn mass(&self) -> f64 {
        4.867e24
    }
}

impl SolarObject for Earth {
    fn coords(&self) -> Vector3 {
        let coords = vsop87a::earth(self.time.0);
        Vector3::from((coords.x, coords.y, coords.z))
    }
    
    fn tick(&mut self, delta_t: f64) {
        self.time.0 += delta_t;
    }

    fn name(&self) -> String {
        return "Earth".to_string();
    }
    
    fn mass(&self) -> f64 {
        5.972e24
    }
}

impl SolarObject for Mars {
    fn coords(&self) -> Vector3 {
        let coords = vsop87a::mars(self.time.0);
        Vector3::from((coords.x, coords.y, coords.z))
    }
    
    fn tick(&mut self, delta_t: f64) {
        self.time.0 += delta_t;
    }

    fn name(&self) -> String {
        return "Mars".to_string();
    }
    
    fn mass(&self) -> f64 {
        6.39e23
    }
}

impl SolarObject for Saturn {
    fn coords(&self) -> Vector3 {
        let coords = vsop87a::saturn(self.time.0);
        Vector3::from((coords.x, coords.y, coords.z))
    }
    
    fn tick(&mut self, delta_t: f64) {
        self.time.0 += delta_t;
    }

    fn name(&self) -> String {
        return "Saturn".to_string();
    }
    
    fn mass(&self) -> f64 {
        5.683e26
    }
}

impl SolarObject for Jupiter {
    fn coords(&self) -> Vector3 {
        let coords = vsop87a::jupiter(self.time.0);
        Vector3::from((coords.x, coords.y, coords.z))
    }
    
    fn tick(&mut self, delta_t: f64) {
        self.time.0 += delta_t;
    }

    fn name(&self) -> String {
        return "Jupiter".to_string();
    }
    
    fn mass(&self) -> f64 {
        1.89813e27
    }
}

impl SolarObject for Uranus {
    fn coords(&self) -> Vector3 {
        let coords = vsop87a::uranus(self.time.0);
        Vector3::from((coords.x, coords.y, coords.z))
    }
    
    fn tick(&mut self, delta_t: f64) {
        self.time.0 += delta_t;
    }

    fn name(&self) -> String {
        return "Uranus".to_string();
    }
    
    fn mass(&self) -> f64 {
        8.681e25
    }
}

impl SolarObject for Neptune {
    fn coords(&self) -> Vector3 {
        let coords = vsop87a::neptune(self.time.0);
        Vector3::from((coords.x, coords.y, coords.z))
    }
    
    fn tick(&mut self, delta_t: f64) {
        self.time.0 += delta_t;
    }

    fn name(&self) -> String {
        return "Neptune".to_string();
    }
    
    fn mass(&self) -> f64 {
        1.024e26
    }
}