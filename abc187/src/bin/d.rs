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
        mut ab: [(i64, i64); n]
    }
    let mut res = 0;
    ab.sort_by(|x, y| (y.1 + y.0 * 2).partial_cmp(&(x.1 + x.0 * 2)).unwrap());
    let mut a_sum = ab.iter().fold(0, |sum, (a, _b)| sum + a);
    let mut takahashi_sum = 0;
    for (a, b) in ab {
        res += 1;
        takahashi_sum += a + b;
        a_sum -= a;
        if takahashi_sum > a_sum {
            break;
        }
    }
    println!("{}", res);
}
