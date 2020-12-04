use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

type V3 = [f64; 3];

const Q_M: f64 = 1.0;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

fn add(a: &V3, b: &V3) -> V3 { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
fn subtract(a: &V3, b: &V3) -> V3 { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn mul(a: f64, b: &V3) -> V3 { [a*b[0], a*b[1], a*b[2]] }
fn cross(a: &V3, b: &V3) -> V3 { [a[1]*b[2] - b[1]*a[2], a[2]*b[0] - b[2]*a[0], a[0]*b[1] - b[0]*a[1]] }
fn dot(a: &V3, b: &V3) -> f64 { a.iter().zip(b.iter()).map(|(x,y)| x*y).sum() }

#[wasm_bindgen]
pub struct Trajectory {
    states: Vec<State>
}

#[wasm_bindgen]
#[derive(Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub b0: f64,
    pub l: f64,
    pub vel_par: f64,
    pub vel_perp: f64,
    pub n_steps: usize,
}

#[wasm_bindgen]
impl Config {
    pub fn empty() -> JsValue {
        let c: Config = Default::default();
        JsValue::from_serde(&c).unwrap()
    }

    pub fn from_struct(val: &JsValue) -> Self {
        val.into_serde().unwrap()
    }
}



#[wasm_bindgen]
impl Trajectory {
    pub fn new(c: Config) -> Self {
        let dt = 0.08 * c.b0 * Q_M;
        Self::try_new(dt, c)
    }

    pub fn try_new(dt: f64, c: Config) -> Self {
        log(format!("dt = {}, n = {}", dt, c.n_steps).as_str());
        let mut traj = Vec::with_capacity(c.n_steps+1);
        let pos = [-c.vel_perp/Q_M/c.b0, 0., 0.];
        let init = State { t: 0.0, pos, vel: [0., c.vel_perp, c.vel_par], field_mag: 0.0 }; 
        let mut s = init.clone();
        traj.push(init);
        for _i in 0..c.n_steps {
            s = s.step(dt, c.b0, c.l);
            if 0.1 / s.field_mag * Q_M < dt {
                return Self::try_new(dt/2.0, c);
            }
            traj.push(s.clone());
        }
        Self { states: traj }
    }


    pub fn at(&self, i: usize) -> State {
        self.states[i]
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub struct State {
    pos: V3,
    vel: V3,
    pub t: f64,
    pub field_mag: f64,
}

#[wasm_bindgen]
impl State {
    pub fn x(&self) -> f64 { self.pos[0] }
    pub fn y(&self) -> f64 { self.pos[1] }
    pub fn z(&self) -> f64 { self.pos[2] }
    pub fn vx(&self) -> f64 { self.vel[0] }
    pub fn vy(&self) -> f64 { self.vel[1] }
    pub fn vz(&self) -> f64 { self.vel[2] }

    pub fn step(&self, dt: f64, b0: f64, l: f64) -> Self {
        let mag_field = mul(b0, &mag_field_1(&self.pos, l));
        let field_mag = dot(&mag_field, &mag_field).sqrt();
        let vel = boris(&self.vel, dt, Q_M, &mag_field);
        let pos = add(&self.pos, &mul(dt, &vel));
    
        Self { t: self.t + dt, pos, vel, field_mag }
    }

    pub fn mag_moment(&self) -> f64 {
        // m = 1.0
        0.5 / self.field_mag * (self.vx() * self.vx() + self.vy() * self.vy())
    }

    pub fn energy(&self) -> f64 {
        // m = 1.0
        0.5 * (self.vx() * self.vx() + self.vy() * self.vy() + self.vz() * self.vz())
    }

    pub fn v_perp(&self) -> f64 {
        self.vx() * self.vx() + self.vy() * self.vy()
    }
}


fn boris(v: &V3, dt: f64, q_m: f64, mag_field: &V3) -> V3
{
    let b = mul(q_m * dt / 2., mag_field);
    let b2 = dot(&b, &b);
    let mut res = mul(dot(&v, &b), &b);
    res = add(&cross(&v, &b), &res);
    res = add(&v, &res);
    res = mul(2./(1. + b2), &res);
    subtract(&res, &v)
//    subtract(&mul(2./(1. + b2), &add(&add(v, &cross(v, &b)), &mul(dot(v, &b), &b))), v)
}


fn mag_field_1(r: &V3, l: f64) -> V3 {
    let (x, y, z) = (r[0], r[1], r[2]);
    [-x*z/l/l, -y*z/l/l, 1. + z*z/l/l] 
}

#[wasm_bindgen]
pub fn psi_1(s: &State, c: &Config) -> f64 {
    let r = &s.pos;
    c.b0 * (r[0]*r[0] + r[1]*r[1]) * (1.0 + r[2]*r[2] / c.l / c.l)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn mag_f() {
        let r0 = [-0.0195, 0.0128, 0.0064];
        let f = mag_field_1(&r0, 0.1);
        println!("{} {} {}", f[0], f[1], f[2]);
        println!("{}", dot(&f, &f).sqrt());
        println!("{}", -r0[0] * r0[2] / 0.1 / 0.1);
        assert!(true);
    }
}
