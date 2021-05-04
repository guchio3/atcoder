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
        n: usize
    }
    let mut dp = vec![100_000; 100_000 + 1];
    dp[0] = 0;

    let mut nums = vec![1];
    for i in 1..=6 {
        nums.push(6usize.pow(i));
    }
    for i in 1..=5 {
        nums.push(9usize.pow(i));
    }
    nums.sort();
    for i in 1..=100_000 {
        for j in 0..nums.len() {
            let num = nums[j];
            if i >= num {
                dp[i] = min(dp[i], dp[i - num] + 1);
            }
        }
    }

    println!("{}", dp[n]);
}
