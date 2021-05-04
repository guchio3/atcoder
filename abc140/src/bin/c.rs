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
        b: [usize; n - 1]
    }
    let mut bef_b_i = b[0];
    let mut res = 0;
    for i in 0..n - 1 {
        let b_i = b[i];
        res += min(b_i, bef_b_i);
        bef_b_i = b[i];
    }
    res += bef_b_i;
    println!("{}", res);
}
