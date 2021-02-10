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
        mut ab: [(usize, usize); n]
    }
    ab = ab.into_iter().rev().collect();
    let mut res = 0;
    for (mut a_i, b_i) in ab {
        a_i += res;
        if a_i % b_i != 0 {
            res += b_i - (a_i % b_i);
        }
    }
    println!("{}", res);
}
