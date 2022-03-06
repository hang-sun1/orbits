mod utils;
mod planets;
mod kepler;
mod vector3;

use core::borrow;
use std::{rc::Rc, cell::RefCell};

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
static DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

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
    solar_objects: Vec<Rc<RefCell<dyn SolarObject>>>,
    time: JulianDate
}

#[wasm_bindgen]
impl SolSystem {
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let time = JulianDate(2459642.5);

        let mut solar_objects: Vec<Rc<RefCell<dyn SolarObject>>> = Vec::new();

        let sol = Rc::new(RefCell::new(Sol));
        let sol_mass = sol.borrow().mass();

        let mercury = Rc::new(RefCell::new(Mercury { time }));
        let venus = Rc::new(RefCell::new(Venus { time }));
        let earth = Rc::new(RefCell::new(Earth { time }));
        let mars = Rc::new(RefCell::new(Mars { time }));
        let jupiter = Rc::new(RefCell::new(Jupiter { time }));
        let saturn = Rc::new(RefCell::new(Saturn { time }));
        let uranus = Rc::new(RefCell::new(Uranus { time }));
        let neptune = Rc::new(RefCell::new(Neptune { time }));
        
        solar_objects.push(sol.clone());
        solar_objects.push(mercury.clone());
        solar_objects.push(venus.clone());
        solar_objects.push(earth.clone());
        solar_objects.push(mars.clone());
        solar_objects.push(jupiter.clone());
        solar_objects.push(saturn.clone());
        solar_objects.push(uranus.clone());
        solar_objects.push(neptune.clone());
        
        let keps = KeplerParams::new(
            9.340419574613645E-02,
            2.279286491077153E+11 / 1.49597870691e11,
            2.867429735262922E+02 * DEG_TO_RAD,
            4.949033037641041E+01 * DEG_TO_RAD,
            1.847932354966402E+00 * DEG_TO_RAD,
            3.025677836626235E+02 * DEG_TO_RAD,
        );
        
        let mars_kep = Rc::new(RefCell::new(KeplerBody::new("mars-kep".to_string(), keps, 12.0, sol.clone(), time)));

        let moon_params = KeplerParams::new(
            5.391394384104019E-02,
            2.582818179985506E-03,
            2.278613677423641E+02 * DEG_TO_RAD,
            5.489485645757891E+01 * DEG_TO_RAD,
            5.029332310855371E+00 * DEG_TO_RAD,
            7.038869476425367E+01 * DEG_TO_RAD,
        );

        let moon_kep = Rc::new(RefCell::new(KeplerBody::new("Luna-kep".to_string(), moon_params, 11.0, earth.clone(), time)));

        solar_objects.push(mars_kep.clone());
        solar_objects.push(moon_kep.clone());
        
        Self {
            solar_objects,
            time,
        }
    }

    pub fn tick(&mut self, delta_t: f64) {
        self.time.0 += delta_t;
        self.solar_objects.iter_mut()
            .for_each(|o| {
                o.borrow_mut().tick(delta_t);
            });
    }

    pub fn positions(&self) -> JsValue {
        let coords = self.solar_objects.iter()
            .map(|o| o.borrow().coords().into())
            .collect();

        let names = self.solar_objects.iter()
            .map(|o| o.borrow().name())
            .collect();

        let info = SolInfo {
            time: self.time.0,
            coords,
            names,
        };

        JsValue::from_serde(&info).unwrap()
    }

    pub fn add_keplerian(&mut self, name: String, mass: f64, central_body_name: String, e: f64, a: f64, w: f64, U: f64, i: f64, M: f64) {
        let params = KeplerParams::new(e, a, w, U, i, M);
        let central_body = self.solar_objects.iter().find(|e| {
            e.borrow().name() == central_body_name
        });
        
        if let Some(o) = central_body {
            let new_kepler: Rc<RefCell<dyn SolarObject + 'static>> = Rc::new(RefCell::new(KeplerBody::new(name, params, mass, o.clone(), self.time)));
            self.solar_objects.push(new_kepler.clone());
        }
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