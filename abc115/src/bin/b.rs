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
        mut p: [usize; n]
    }
    p.sort();
    let mut res = 0;
    for i in 0..n - 1 {
        res += p[i];
    }
    res += p[n - 1] / 2;
    println!("{}", res);
}
