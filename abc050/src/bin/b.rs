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
        t: [usize; n],
        m: usize,
        px: [(usize, usize); m]
    }
    let t_sum: usize = t.iter().map(|x| *x).sum();
    for (pi, xi) in px {
        let res = t_sum - t[pi - 1] + xi;
        println!("{}", res);
    }
}
