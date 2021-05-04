#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use ordered_float::NotNan;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize, d: f64, h: f64,
        dh: [(f64, f64); n]
    }
    let mut res = 0.;
    for (d_i, h_i) in dh {
        if res < h - (h_i - h) / (d_i - d) * d {
            res = h - (h_i - h) / (d_i - d) * d;
        }
    }
    println!("{}", res);
}
