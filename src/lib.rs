extern crate num;
use num::Float;

pub mod floptimizer;

#[cfg(test)]
mod tests {
    use crate::{test_flat_fn, test_opt_fn};
    use crate::floptimizer::{Floptimizer, FloptiBuilder};

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

    #[test]
    fn builder_test() {
        let flopti = FloptiBuilder::new()
            .unimodal(true)
            .set_function(test_flat_fn)
            .upper_bound(10.)
            .lower_bound(10.)
            .build()
            .expect("Could not create floptimizer");


        assert_eq!((flopti.opt_fn)(flopti.flat_fn),2.);


    }
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
