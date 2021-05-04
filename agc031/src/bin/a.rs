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
        n: usize,
        s: Chars
    }
    let mut res = 1;
    let mut cnt = vec![0; 26];
    for i in 0..n {
        cnt[(s[i] as u8 - 'a' as u8) as usize] += 1;
    }
    for i in 0..26 {
        res = (res * (cnt[i] + 1)) % MOD;
    }
    res -= 1;
    println!("{}", res);
}

// fn main() {
//     input! {
//         n: usize,
//         s: Chars
//     }
//     let mut res = 0;
//     let mut start = 0;
//     let mut end = 0;
//     let mut cnt = vec![0; 26];
//     while end < n {
//         while cnt[(s[end] as u8 - 'a' as u8) as usize] > 0 {
//             cnt[(s[start] as u8 - 'a' as u8) as usize] -= 1;
//             start += 1;
//         }
//         cnt[(s[end] as u8 - 'a' as u8) as usize] += 1;
//         res = (res + end - start + 1) % MOD;
//         end += 1;
//         println!("{:?}", cnt);
//     }
//     println!("{}", res);
// }
