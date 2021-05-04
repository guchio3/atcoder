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
        mut a: [usize; n + 1],
        b: [usize; n]
    }
    let init_sum: usize = a.iter().sum::<usize>();
    for i in 0..n {
        if a[i] >= b[i] {
            a[i] -= b[i];
        } else {
            let diff = b[i] - a[i];
            a[i] = 0;
            a[i + 1] -= min(diff, a[i + 1]);
        }
    }
    let last_sum: usize = a.iter().sum::<usize>();
    println!("{}", init_sum - last_sum);
}

// fn main() {
//     input! {
//         n: usize,
//         a: [usize; n + 1],
//         b: [usize; n]
//     }
//     let mut dp = vec![vec![0; 2]; n];
//     dp[0][0] = min(a[0], b[0]);
//     for i in 1..n {
//         dp[i][0] = max(
//             dp[i - 1][0] + min(a[i], b[i]),
//             dp[i - 1][1] + min(a[i], b[i] + b[i - 1]),
//         );
//         dp[i][1] = max(dp[i - 1][0], dp[i - 1][1] + min(a[i], b[i - 1]));
//     }
//     let res = max(dp[n - 1][0], dp[n - 1][1] + min(a[n], b[n - 1]));
//     println!("{}", res);
// }
