use ordered_float::NotNan;
use proconio::{fastout, input};
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n]
    }

    let mut bh = BinaryHeap::new();
    for a_i in a {
        bh.push(NotNan::new(a_i as f64).unwrap());
    }

    for _i in 0..m {
        let mut top_a = bh.pop().unwrap();
        top_a /= 2.;
        bh.push(top_a);
    }

    let res = bh.iter().fold(0, |s, x| x.into_inner() as usize + s);
    println!("{}", res);
}
