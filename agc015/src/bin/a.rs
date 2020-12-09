#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize, a: usize, b: usize
    }
    let max_sum = b * (n - 1) + a;
    let min_sum = a * (n - 1) + b;
    // let max_sum = (b + b - n - 1 + 1) * (n - 1) / 2 + a;
    // let min_sum = (a + a + n - 1 - 1) * (n - 1) / 2 + b;
    // println!("{} {}", max_sum, min_sum);
    if a > b || (a != b && n == 1) {
        println!("0");
    } else {
        println!("{}", max_sum + 1 - min_sum);
    }
}
