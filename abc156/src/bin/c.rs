#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::usize::MAX;

fn main() {
    input! {
        n: usize,
        x: [usize; n]
    }
    let mut res = MAX;
    for p in 1..=100 {
        let mut tmp_res = 0;
        for &x_i in x.iter() {
            tmp_res += (max(x_i, p) - min(x_i, p)).pow(2);
        }
        res = min(res, tmp_res);
    }
    println!("{}", res);
}
