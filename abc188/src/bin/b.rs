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
        a: [i64; n],
        b: [i64; n]
    }
    let mut res = 0;
    for i in 0..n {
        res += a[i] * b[i];
    }
    if res == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
