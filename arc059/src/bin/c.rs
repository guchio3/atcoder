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
    for i in -100..=100 {
        let mut tmp_res = 0;
        for j in 0..n {
            let a_j = a[j];
            tmp_res += (i - a_j).pow(2);
        }
        res = min(res, tmp_res);
    }
    println!("{}", res);
}
