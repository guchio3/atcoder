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
        a: [usize; n]
    }
    let mut res = 0;
    for l in 0..n {
        let mut min_num = 1_000_000_000;
        for r in l..n {
            min_num = min(min_num, a[r]);
            res = max(res, (r - l + 1) * min_num);
        }
    }
    println!("{}", res);
}
