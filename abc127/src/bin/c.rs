use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        lr: [(usize, usize); m]
    }
    let mut max_l = 0;
    let mut min_r = n;
    for (l_i, r_i) in lr {
        max_l = max(max_l, l_i);
        min_r = min(min_r, r_i);
    }
    if max_l > min_r {
        println!("0");
    } else {
        println!("{}", min_r - max_l + 1);
    }
}
