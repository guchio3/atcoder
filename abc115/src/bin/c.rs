#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize, k: usize,
        mut h: [i64; n]
    }
    h.sort();

    let mut min_i = 0;
    let mut max_i = k - 1;
    let mut res: i64 = 1_000_000_000;
    for _i in k - 1..n {
        res = min(res, h[max_i] - h[min_i]);
        min_i += 1;
        max_i += 1;
    }
    println!("{}", res);
}
