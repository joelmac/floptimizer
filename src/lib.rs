extern crate num;
use num::Float;
#[cfg(test)]
mod tests {
    use crate::{test_flat_fn, test_opt_fn, Floptimizer};

    #[test]
    fn test_floptimizer() {
        let floppy = Floptimizer{
            range_bot:0.,
            range_top:0.,
            flat_fn: test_flat_fn,
            opt_fn: test_opt_fn,
        }; 
        println!("{:?}", floppy);
        assert_eq!((floppy.opt_fn)(floppy.flat_fn),2.);
    }
}

#[derive(Debug)]
struct Floptimizer {
    range_bot: f32,
    range_top: f32,
    flat_fn: fn(f32) -> f32,
    opt_fn: fn(fn(f32) -> f32) -> f32, 
}

fn test_flat_fn (x: f32) -> f32 {
    -(x-2.)*(x-2.)
}

fn test_opt_fn (input_f: fn(f32) -> f32) -> f32{
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
