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
        mut apx: [(usize, usize, usize); n]
    }
    apx.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
    for (ai, pi, xi) in apx {
        if xi > ai {
            println!("{}", pi);
            return;
        }
    }
    println!("-1");
}
