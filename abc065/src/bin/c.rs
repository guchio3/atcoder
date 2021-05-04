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
        n: usize, m: usize
    }
    let mut res = 1;
    if (max(n, m) - min(n, m)) == 0 {
        for i in 1..=n {
            res = (res * i) % MOD;
        }
        for i in 1..=m {
            res = (res * i) % MOD;
        }
        res = (res * 2) % MOD;
    } else if (max(n, m) - min(n, m)) == 1 {
        for i in 1..=n {
            res = (res * i) % MOD;
        }
        for i in 1..=m {
            res = (res * i) % MOD;
        }
    } else {
        res = 0;
    }
    println!("{}", res);
}
