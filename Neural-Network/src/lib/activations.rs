use std::f64::consts::E;

#[derive(Clone)]
pub struct Activation<'a> {
    pub func: &'a dyn Fn(f64) -> f64,
    pub dfunc: &'a dyn Fn(f64) -> f64,
}

pub const SIGMOID: Activation = Activation {
    func: &|x: f64| -> f64 { 1.0 / (1.0 + E.powf(-x)) },
    dfunc: &|x: f64| -> f64 { x * (1.0 - x) },
};
