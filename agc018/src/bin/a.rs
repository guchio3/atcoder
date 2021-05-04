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
        n: usize, k: i64,
        a: [i64; n]
    }
    let mut max_num = a[0];
    let mut gcd_num = a[0];
    for i in 1..n {
        gcd_num = gcd(gcd_num, a[i]);
        max_num = max(max_num, a[i]);
    }
    if k <= max_num && k % gcd_num == 0 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
