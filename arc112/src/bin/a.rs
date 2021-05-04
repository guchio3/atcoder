#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

#[fastout]
fn main() {
    input! {
        t: usize,
        lr: [(usize, usize); t]
    }
    for (l_i, r_i) in lr {
        if r_i < l_i * 2 {
            println!("0");
        } else {
            let res = (r_i - 2 * l_i + 1) * (r_i - l_i * 2 + 2) / 2;
            println!("{}", res);
        }
    }
}
