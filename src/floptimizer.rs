extern crate num;
use num::Float;

#[derive(Debug)]
pub struct Floptimizer {
    pub flat_fn: fn(f32) -> f32,
    pub opt_fn: fn(fn(f32) -> f32) -> f32, 
    pub range_bot: f32,
    pub range_top: f32,
}


fn empty_func(x: f32) -> f32 {
    0.
}

fn empty_search_func(input_f: fn(f32) -> f32) -> f32 {
    0.
}

fn golden_search_function(input_f: fn(f32) -> f32) -> f32{
    // step1
    let t = 0.000001;
    let mut a = -1.0; //
    let mut b = 5.0;
    let phi = (1. + (5.).sqrt())/2.;
    // step2
    let mut c = b - (b - a) / phi;
    let mut d = a + (b - a) / phi;

    while (b - a) > t {

        // step3
        match input_f(c) < input_f(d) {
            true => {a = c; 
                    c = d;
                    d = a + (b - a) / phi;}
            false => {b = d;
                    d = c;
                    c = b - (b - a) / phi;}
        }
    }
    (b + a)/2.
}

pub struct FloptiBuilder {
    pub flat_fn: fn(f32) -> f32,
    pub unimodal: bool,
    pub range_bot: f32,
    pub range_top: f32,
}

impl FloptiBuilder {

    pub fn new() -> Self {
        FloptiBuilder {
            flat_fn: empty_func,
            unimodal: false,
            range_bot: -100.,
            range_top: 100.,
        }
    }

    pub fn unimodal(mut self, val: bool) -> Self {
        self.unimodal = val;
        self
    }

    pub fn lower_bound(mut self, val: f32) -> Self {
        self.range_bot = val;
        self
    }

    pub fn upper_bound(mut self, val: f32) -> Self {
        self.range_top = val;
        self
    }

    pub fn set_function(mut self, input_fn: fn(f32) -> f32) -> Self {
        self.flat_fn = input_fn;
        self
    }

    pub fn build(&self) -> Result<Floptimizer,String> {
        let opt_fn = match self.unimodal {
            false => {
                return Err("Sorry but only unimodal functions are supported at this time. Please mark the function as unimodal by using the builder method .unimodal(true)".to_string());
                empty_search_func
            }
            true => {
                golden_search_function
            }
        };
        let floptimizer = Floptimizer {
            flat_fn: self.flat_fn,
            opt_fn: opt_fn,
            range_bot: self.range_bot,
            range_top: self.range_top,
        };

        Ok(floptimizer)


    }
}




