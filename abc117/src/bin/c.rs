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
        n: usize, m: usize,
        mut x: [i64; m]
    }
    x.sort();
    if n >= m {
        println!("0");
    } else {
        // let mut diffs: Vec<(usize, i64)> = x
        //     .windows(2)
        //     .enumerate()
        //     .into_iter()
        //     .map(|(i, xx)| (i, xx[1] - xx[0]))
        //     .collect();
        let mut diffs: Vec<i64> = x.windows(2).into_iter().map(|xx| xx[1] - xx[0]).collect();
        diffs.sort_by(|a, b| b.partial_cmp(&a).unwrap());
        println!("{}", diffs[n - 1..].iter().sum::<i64>());
    }
}
