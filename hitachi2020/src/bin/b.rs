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
        a: usize, b: usize, m: usize,
        aa: [usize; a],
        bb: [usize; b],
        xyc: [(usize, usize, usize); m]
    }
    let mut res = aa.iter().map(|xx| *xx).min().unwrap() + bb.iter().map(|xx| *xx).min().unwrap();
    for (x, y, c) in xyc {
        res = min(res, aa[x - 1] + bb[y - 1] - c);
    }
    println!("{}", res);
}
