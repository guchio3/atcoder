use proconio::{input, fastout};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [i64; n]
    }
    let mut man = 0;
    let mut yuc = 0;
    let mut che = 0;
    for x_i in x {
        man += x_i.abs();
        yuc += x_i.pow(2);
        che = max(che, x_i.abs());
    }
    let yuc = (yuc as f64).sqrt();
    println!("{}", man);
    println!("{}", yuc);
    println!("{}", che);
}
