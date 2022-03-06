use std::rc::Rc;
use std::cell::RefCell;

use vsop87::KeplerianElements;

use crate::{SolarObject, vector3::Vector3, JulianDate, GRAV};
use ode_solvers::*;

type State = Vector6<f64>;
type Time = f64;

#[derive(Clone, Copy)]
pub(crate) struct KeplerParams {
    pub(crate) eccentricity: f64,
    pub(crate) semimajor_axis: f64,
    pub(crate) periapsis: f64,
    pub(crate) ascending_node: f64,
    pub(crate) inclination: f64,
    pub(crate) mean_anomaly: f64,
}

impl KeplerParams {
    pub(crate) fn new(e: f64, a: f64, w: f64, U: f64, i: f64, M: f64) -> Self {
        Self {
            eccentricity: e,
            semimajor_axis: a,
            periapsis: w,
            ascending_node: U,
            inclination: i,
            mean_anomaly: M,
        }
    }
}

#[derive(Clone)]
pub(crate) struct KeplerBody {
    name: String,
    pos: Vector3,
    vel: Vector3,
    mass: f64,
    central_body: Rc<RefCell<dyn SolarObject>>,
    time: JulianDate,
}

impl KeplerBody {
    pub(crate) fn new(name: String, params: KeplerParams, mass: f64, central_body: Rc<RefCell<dyn SolarObject>>, time: JulianDate) -> Self {
        let (pos, vel) = KeplerBody::kepler_to_state_vectors(params, central_body.borrow().mass());
        KeplerBody {
            name,
            pos,
            vel,
            mass,
            central_body,
            time,
        }
    }

    fn kepler_to_state_vectors(elements: KeplerParams, central_mass: f64) -> (Vector3, Vector3) {
        let e = elements.eccentricity;
        let a = elements.semimajor_axis; // * 1.49597870691e11;
        let mu = central_mass * GRAV;
        let omega = elements.periapsis; // + 2.0*std::f64::consts::PI;
        let Omega = elements.ascending_node;
        let i = elements.inclination;


        let M = elements.mean_anomaly; //  - 2.0*std::f64::consts::PI;
        let E = KeplerBody::eccentric_anomaly(e, M);
        let nu = 2.0 * ((1.0+e).sqrt() * (E/2.0).sin()).atan2((1.0-e).sqrt() * (E/2.0).cos());
        let r = a * (1.0 - e * E.cos());

        let o = r*Vector3(nu.cos(), nu.sin(), 0.0);
        let odot =  ((mu*a).sqrt()/r) * Vector3(-1.0*E.sin(), (1.0-e*e).sqrt() * E.cos(), 0.0);

        let rx = o.0 * (omega.cos()*Omega.cos() - omega.sin()*i.cos()*Omega.sin()) - o.1 * (omega.sin()*Omega.cos() + omega.cos()*i.cos()*Omega.sin());
        let ry = o.0 * (omega.cos()*Omega.sin() + omega.sin()*i.cos()*Omega.cos()) + o.1 * (omega.cos()*i.cos()*Omega.cos()-omega.sin()*Omega.sin());
        let rz = o.0 * (omega.sin()*i.sin()) + o.1*(omega.cos()*i.sin());
        let r3d = Vector3(rx, ry, rz);
        

        let rdotx = odot.0 * (omega.cos() * Omega.cos() - omega.sin()*i.cos()*Omega.sin()) - odot.1 * (omega.sin()*Omega.cos()+omega.cos()*i.cos()*Omega.sin());
        let rdoty = odot.0 * (omega.cos()*Omega.sin() + omega.sin()*i.cos()*Omega.cos())  + odot.1 * (omega.cos()*i.cos()*Omega.cos()-omega.sin()*Omega.sin());
        let rdotz = odot.0 * (omega.sin()*i.sin()) + odot.1*(omega.cos()*i.sin());
        let rdot = Vector3(rdotx, rdoty, rdotz);

        (r3d, rdot)
    }

    fn eccentric_anomaly(e: f64, M: f64) -> f64 {
        let mut E = M;
        let mut delta_E = 100.0f64;
        let mut delta_M = 100.0f64;

        while (delta_E.abs() > 1e-6) {
            delta_M = M - (E - e * E.sin());
            delta_E = delta_M / (1.0 - e * E.cos());
            E += delta_E;
        }
        
        // for _ in 0..5 {
        //     E = E - (E - e*E.sin() - M) / (1.0 - e * E.cos());
        // }
        E
    }

}

impl SolarObject for KeplerBody {
    fn coords(&self) -> Vector3 {
        self.pos + self.central_body.borrow().coords()
    }

    fn tick(&mut self, delta_t: f64) {
        let inputs = Vector6::new(self.pos.0, self.pos.1, self.pos.2,self.vel.0, self.vel.1, self.vel.2);
        let mut stepper = Rk4::new(self.clone(), self.time.0, inputs, self.time.0 + delta_t, 1e-2);
        
        let _ = stepper.integrate().unwrap();
        let res = stepper.y_out();

        
        let last = res[res.len()-1];
        self.pos = Vector3(last[0], last[1], last[2]);
        self.vel = Vector3(last[3], last[4], last[5]);

        self.time.0 += delta_t; 
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn mass(&self) -> f64 {
        self.mass
    }
}

impl ode_solvers::System<State> for KeplerBody {
    fn system(&self, _x: f64, y: &State, dy: &mut State) {
        let r = (y[0] * y[0] + y[1] * y[1] + y[2] * y[2]).sqrt();

        dy[0] = y[3];
        dy[1] = y[4];
        dy[2] = y[5];

        let central_mass = self.central_body.borrow().mass();
        dy[3] = -central_mass * GRAV * y[0] / r.powi(3);
        dy[4] = -central_mass * GRAV * y[1] / r.powi(3);
        dy[5] = -central_mass * GRAV * y[2] / r.powi(3);
    }
}

#[cfg(test)]
mod tests {
    use vsop87::{vsop87a, KeplerianElements};
    use more_asserts::*;
    use crate::*;

    #[test]
    fn kepler_to_state_vector_works() {
        let sys = SolSystem::new();
        let deg_to_rad = std::f64::consts::PI / 180.0;
        let params = KeplerParams::new(
            9.340419574613645E-02,
            2.279286491077153E+11,
            2.867429735262922E+02 * deg_to_rad,
            4.949033037641041E+01 * deg_to_rad,
            1.847932354966402E+00 * deg_to_rad,
            3.025677836626235E+02 * deg_to_rad,
        );

        let (pos, vel) = KeplerBody::kepler_to_state_vectors(params, Sol {}.mass());

        assert!((pos.0 - -2.013461427639138E-02).abs() < 0.00001, "x = {}", pos.0);
        assert!((pos.1 - -1.456710381715603).abs() < 0.00001, "y = {}", pos.1);
        assert!((pos.2 - -3.003544415806017E-02).abs() < 0.00001, "z = {}", pos.2);
        assert!((vel.0 - 1.451891634280422E-02).abs() < 0.00001, "vx = {}", vel.0);
        assert!((vel.1 - 1.010174565114862E-03).abs() < 0.00001, "vy = {}", vel.1);
        assert!((vel.2 - -3.349775733915481E-04).abs() < 0.00001, "vz = {}", vel.2);
    }
}

