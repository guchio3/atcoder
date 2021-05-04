#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut res = 1;
    let mut cnts = vec![0; n + 1];
    cnts[0] = 3;
    for i in 0..n {
        res = (res * cnts[a[i]]) % MOD;
        cnts[a[i]] -= 1;
        cnts[a[i] + 1] += 1;
    }
    println!("{}", res);
}

// fn main() {
//     input! {
//         n: usize,
//         a: [usize; n]
//     }
//     if a[0] != 0 {
//         println!("0");
//         return;
//     }
//     let mut dp = vec![0; n];
//     let mut cnts = vec![0; n + 1];
//     cnts[0] = 2;
//     cnts[1] = 1;
//     dp[0] = 3;
//     for i in 1..n {
//         // if cnts[a[i]] == 0 || cnts[a[i] + 1] == 3 {
//         //     println!("0");
//         //     return;
//         // }
//         dp[i] = (dp[i - 1] * cnts[a[i]]) % MOD;
//         cnts[a[i]] -= 1;
//         cnts[a[i] + 1] += 1;
//     }
//     println!("{}", dp[n - 1]);
// }
