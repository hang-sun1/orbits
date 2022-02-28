#![feature(box_syntax)]

mod utils;
mod planets;
mod vector3;

use serde::{Serialize, Deserialize};
use vector3::Vector3;
use wasm_bindgen::prelude::*;

use planets::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[derive(Clone, Copy)]
struct JulianDate(f64);

#[derive(Serialize, Deserialize)]
pub struct SolInfo {
    pub time: f64,
    pub coords: Vec<(f64, f64, f64)>,
    pub names: Vec<String>,
}

#[wasm_bindgen]
pub struct SolSystem {
    solar_objects: Vec<Box<dyn SolarObject>>,
    time: JulianDate
}

#[wasm_bindgen]
impl SolSystem {
    pub fn new() -> Self {
        let time = JulianDate(2459638.0);

        let mut solar_objects: Vec<Box<dyn SolarObject>> = Vec::new();
        
        solar_objects.push(box Sol{});
        solar_objects.push(box Mercury{ time });
        solar_objects.push(box Venus{ time });
        solar_objects.push(box Earth{ time });
        solar_objects.push(box Mars{ time });
        solar_objects.push(box Jupiter{ time });
        solar_objects.push(box Saturn{ time });
        solar_objects.push(box Uranus{ time });
        solar_objects.push(box Neptune{ time });
        
        Self {
            solar_objects,
            time,
        }
    }

    pub fn tick(&mut self, delta_t: f64) {
        self.time.0 += delta_t;
        self.solar_objects.iter_mut()
            .for_each(|o| {
                o.tick(delta_t);
            });
    }

    pub fn positions(&self) -> JsValue {
        let coords = self.solar_objects.iter()
            .map(|o| o.coords().into())
            .collect();

        let names = self.solar_objects.iter()
            .map(|o| o.name())
            .collect();

        let info = SolInfo {
            time: self.time.0,
            coords,
            names,
        };

        JsValue::from_serde(&info).unwrap()
    }

    pub fn width(&self) -> u32 {
        50
    }
    pub fn height(&self) -> u32 {
        50
    }
}

/*
    A trait that describes an object in the solar system.
    Includes planets, dwarf planets, sol, and anything else
    being modeled.
*/
trait SolarObject {
    // get the current cartesian heliocentric coordinates of the solar object as a 3d vector
    fn coords(&self) -> Vector3;
    
    // move the object in time by delta_t
    fn tick(&mut self, delta_t: f64);

    // return the name of the solar object
    fn name(&self) -> String;

    // returns the mass of the solar object in kg
    fn mass(&self) -> f64;
}

struct Sol;

impl SolarObject for Sol {
    fn coords(&self) -> Vector3 {
        Vector3(0.0, 0.0, 0.0)
    }

    fn tick(&mut self, delta_t: f64) {
        return
    }

    fn name(&self) -> String {
        "Sol".to_string()
    }

    fn mass(&self) -> f64 {
        1.989e30
    }
}