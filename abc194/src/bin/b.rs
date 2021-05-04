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
        ab: [(usize, usize); n]
    }
    let mut res = 1_000_000_000;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                res = min(res, ab[i].0 + ab[j].1);
            } else {
                res = min(res, max(ab[i].0, ab[j].1));
            }
        }
    }
    println!("{}", res);
}
