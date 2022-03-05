#![feature(box_syntax)]

mod utils;
mod planets;
mod kepler;
mod vector3;

use serde::{Serialize, Deserialize};
use utils::set_panic_hook;
use vector3::Vector3;
use vsop87::KeplerianElements;
use wasm_bindgen::prelude::*;

use planets::*;
use kepler::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static GRAV: f64 = 1.488e-34;

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
        console_error_panic_hook::set_once();
        let time = JulianDate(2459642.5);

        let mut solar_objects: Vec<Box<dyn SolarObject>> = Vec::new();

        let sol = Sol;
        let sol_mass = sol.mass();

        let mars = Mars { time };
        let p = mars.coords();
        
        solar_objects.push(box sol);
        // solar_objects.push(box Mercury{ time });
        // solar_objects.push(box Venus{ time });
        // solar_objects.push(box Earth{ time });
        solar_objects.push(box mars);
        // solar_objects.push(box Jupiter{ time });
        // solar_objects.push(box Saturn{ time });
        // solar_objects.push(box Uranus{ time });
        // solar_objects.push(box Neptune{ time });

        // let params: KeplerianElements = vsop87::mars(time.0).into();
        let deg_to_rad = std::f64::consts::PI / 180.0;
        //
        let keps = KeplerParams::new(
            9.340419574613645E-02,
            2.279286491077153E+11,
            2.867429735262922E+02 * deg_to_rad,
            4.949033037641041E+01 * deg_to_rad,
            1.847932354966402E+00 * deg_to_rad,
            3.025677836626235E+02 * deg_to_rad,
        );
        // let keps = KeplerParams {
        //     eccentricity: params.eccentricity(),
        //     semimajor_axis: params.semimajor_axis() * 1.49597870691e11,
        //     periapsis: params.periapsis(),
        //     ascending_node: params.ascending_node(),
        //     inclination: params.inclination(),
        //     mean_anomaly: params.mean_anomaly(),
        // };

        let mars_kep = KeplerBody::new("mars-kep", keps, 12.0, sol_mass, time);

        solar_objects.push(box mars_kep);

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

    fn tick(&mut self, _delta_t: f64) {
        return
    }

    fn name(&self) -> String {
        "Sol".to_string()
    }

    fn mass(&self) -> f64 {
        1.98847e30
    }
}