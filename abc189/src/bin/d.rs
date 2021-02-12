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
        s: [String; n]
    }
    let mut dp: Vec<usize> = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..n {
        let s_i = &s[i];
        if s_i == "AND" {
            dp[i + 1] = dp[i];
        } else {
            dp[i + 1] = dp[i] + (2 << i);
        }
    }
    println!("{}", dp[n]);
}

// fn main() {
//     input! {
//         n: usize,
//         s: [String; n]
//     }
//     let mut flip_res: usize = 1;
//     let mut and_digit = 1;
//     for s_i in s {
//         if s_i == "OR" {
//             flip_res *= (1 << and_digit) - 1;
//             and_digit = 1;
//         } else {
//             and_digit += 1;
//         }
//     }
//     flip_res *= (1 << and_digit) - 1;
//     println!("{}", (2usize << n) - flip_res);
}
