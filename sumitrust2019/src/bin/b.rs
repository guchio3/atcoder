#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: f64,
    }
    let base = (n / 1.08).ceil();
    if (base * 1.08).floor() == n {
        println!("{}", base);
    } else {
        println!(":(");
    }
}
