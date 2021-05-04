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
        s: Chars
    }
    let mut dp = vec![0; s.len()];
    dp[0] = 1;
    for i in 1..s.len() {
        if s[i] != s[i - 1] {
            dp[i] = dp[i - 1] + 1;
        } else {
            if i > 2 {
                dp[i] = dp[i - 3] + 2;
            } else {
                dp[i] = i;
            }
        }
    }
    println!("{}", dp[s.len() - 1]);
}
