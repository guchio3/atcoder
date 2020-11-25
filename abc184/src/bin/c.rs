#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        r1: i32, c1: i32, r2: i32, c2: i32
    }

    let r_diff = (r1 - r2).abs();
    let c_diff = (c1 - c2).abs();
    let point_sum_diff = ((r1 + c1) - (r2 + c2)).abs();
    let point_minus_diff = ((r1 - c1) - (r2 - c2)).abs();
    if r_diff == 0 && c_diff == 0 {
        println!("0");
    } else if r_diff + c_diff <= 3 || point_sum_diff == 0 || point_minus_diff == 0 {
        println!("1");
    } else if (r_diff + c_diff) % 2 == 0
        || point_sum_diff <= 3
        || point_minus_diff <= 3
        || r_diff + c_diff <= 6
    {
        println!("2");
    } else {
        println!("3");
    }
}
