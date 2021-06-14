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
        n: usize, k: i64, m: i64,
        a: [i64; n - 1]
    }
    let mut sum_a = 0;
    for a_i in a {
        sum_a += a_i;
    }
    let diff = n as i64 * m - sum_a;
    if diff > k {
        println!("-1");
    } else if diff < 0 {
        println!("0");
    } else {
        println!("{}", diff);
    }
}
