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
    let mut cumsum: i64 = a.iter().map(|x| *x).sum();
    let mut sq_cumsum: i64 = a.iter().map(|x| (*x) * (*x)).sum();
    let mut res = 0;
    for i in 0..n {
        cumsum -= a[i];
        let a_sq = a[i] * a[i];
        sq_cumsum -= a_sq;
        res += a_sq * (n - i - 1) as i64 + sq_cumsum - 2 * a[i] * cumsum;
    }
    println!("{}", res);
}
