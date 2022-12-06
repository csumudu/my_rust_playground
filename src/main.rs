mod median_mod;
mod unique_mod;

use median_mod::median_mod::median;
use unique_mod::unique_mod::{unique, unique2};

fn main() {
    let mut d = vec![23.0, 3.0, 2., 45f64, 6f64, 9f64];

    let res = median(&mut d);
    println!("Median of {:?} is = {}", d, res.unwrap());

    let v = vec![1, 1, 1, 2, 3, 4, 4, 4];

    let r = unique(&v);
    let r2 = unique2(&v);

    println!("De Duplicated {:?} --> {:?}", v, r);
    println!("De Duplicated v2 {:?} --> {:?}", v, r2);
}
