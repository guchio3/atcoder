#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize
    }
    let mut res = 1;
    for i in 1..=n {
        res = res * i % MOD;
    }
    println!("{}", res);
}
