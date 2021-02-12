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
        n: usize,
        a: [i64; n]
    }
    let mut res = 1_000_000_000_000;
    let mut x = 0;
    let mut y = a.iter().map(|x| *x).sum::<i64>();
    for i in 0..n - 1 {
        x += a[i];
        y -= a[i];
        res = min(res, (x - y).abs());
    }
    println!("{}", res);
}
