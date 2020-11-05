use proconio::{input, fastout};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    println!("{}", max(max(max(a*c, b*d), a*d), b*c));
}
