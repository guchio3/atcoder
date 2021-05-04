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
        n: usize, m: usize,
        a: [usize; m]
    }
    let mut is_invalid = vec![false; n + 1];
    for ai in a {
        is_invalid[ai] = true;
    }

    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..=n {
        if !is_invalid[i] {
            if i >= 1 {
                dp[i] = dp[i - 1] % MOD;
            }
            if i >= 2 {
                dp[i] = (dp[i] + dp[i - 2]) % MOD;
            }
        }
    }
    println!("{}", dp[n]);
}
