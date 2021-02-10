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
        alphabets: [usize; 5]
    }
    let rests: Vec<usize> = alphabets
        .iter()
        .map(|x| *x % 10)
        .filter(|x| *x != 0)
        .collect();
    let mut res;
    if rests.len() == 0 {
        res = alphabets.into_iter().sum::<usize>();
    } else {
        res = alphabets
            .into_iter()
            .map(|x| (x / 10 + (x % 10 != 0) as usize) * 10)
            .sum::<usize>();
        res -= 10;
        res += rests.into_iter().map(|x| x).min().unwrap();
    }
    println!("{}", res);
}

// fn main() {
//     input! {
//         a: usize,
//         b: usize,
//         c: usize,
//         d: usize,
//         e: usize,
//     }
//     let mut res = 0;
//     if a % 10 == 0 && b % 10 == 0 && c % 10 == 0 && d % 10 == 0 && e % 10 == 0 {
//         res += a;
//         res += b;
//         res += c;
//         res += d;
//         res += e;
//     } else {
//         let mut min_res_time = 9;
//         min_res_time = min(min_res_time, a % 10 + ((a % 10 == 0) as usize) * 10);
//         min_res_time = min(min_res_time, b % 10 + ((b % 10 == 0) as usize) * 10);
//         min_res_time = min(min_res_time, c % 10 + ((c % 10 == 0) as usize) * 10);
//         min_res_time = min(min_res_time, d % 10 + ((d % 10 == 0) as usize) * 10);
//         min_res_time = min(min_res_time, e % 10 + ((e % 10 == 0) as usize) * 10);
//         res += (a / 10) * 10 + ((a % 10 != 0) as usize) * 10;
//         res += (b / 10) * 10 + ((b % 10 != 0) as usize) * 10;
//         res += (c / 10) * 10 + ((c % 10 != 0) as usize) * 10;
//         res += (d / 10) * 10 + ((d % 10 != 0) as usize) * 10;
//         res += (e / 10) * 10 + ((e % 10 != 0) as usize) * 10;
//         res -= 10;
//         res += min_res_time;
//     }
//     println!("{}", res);
// }
