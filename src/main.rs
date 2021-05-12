mod sort;
mod math;

use sort::Sort;
use math::pow;
use math::powf;
use math::sqrt;
use math::sqrt_newton;
use math::sqrt_bisection;
use math::log2;

fn main() {
    let mut arr = [ 6, 5, 2, 7, 3, 9, 8, 4, 10, 1 ];
    Sort::quickSort(&mut arr);

    for n in arr.iter() {
        println!("result: {}", n);
    }

    let v = 2;
    let d = 32;
    println!("{}^{} = {}", v, d, pow(v, d));

    let fv = 2.4;
    let fd = 8;
    println!("{}^{} = {}", fv, fd, powf(fv, fd));

    let s = 16;
    println!("sqrt({}) is {}", s, sqrt(s as f32));

    let l = 8;
    println!("log2({}) is {}", l, log2(l));
}
