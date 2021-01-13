#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        w: f64, h: f64, x: f64, y: f64
    }
    let multiple;
    if x == (w / 2.) && y == (h / 2.) {
        multiple = 1;
    } else {
        multiple = 0;
    }
    println!("{} {}", w * h / 2., multiple);
}
