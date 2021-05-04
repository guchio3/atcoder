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
    let mut res = 0.;
    for i in 1..n {
        res += 1. / ((n - i) as f64 / n as f64);
    }
    println!("{}", res);
}

// fn main() {
//     input! {
//         n: usize
//     }
//     let mut prob = 1. / n as f64;
//     for i in 1..=n - 2 {
//         prob *= (n - i) as f64;
//         prob /= n as f64;
//     }
//     let mut perm = 1.;
//     for i in 1..=n - 2 {
//         perm *= i as f64;
//     }
//     let mut res = (n - 1) as f64 * prob * perm * (n - 1) as f64;
//
//     for i in n..n + 10 {
//         prob *= (n - 1) as f64;
//         prob /= n as f64;
//         perm *= i as f64;
//         res += i as f64 * prob * perm * (n - 1) as f64;
//         println!("{}", res);
//         println!("prob: {} / perm: {}", prob, perm);
//     }
//     println!("{}", res);
// }

// fn main() {
//     input! {
//         n: usize
//     }
//     let mut factor = 1;
//     let mut factor_base = 1;
//     for i in 1..n {
//         factor *= i;
//         factor_base *= n;
//     }
//     let mut res = ((n - 1) * factor) as f64;
//     let mut iter_base = 1.;
//     for i in 1..=n - 2 {
//         iter_base *= i as f64;
//     }
//     res /= factor_base as f64;
//     res *= iter_base;
//
//     let factor_mul = factor as f64 / factor_base as f64;
//     let mut now_factor_mul = factor_mul;
//     let mut tmp_res = res;
//     let mut iter_num = 1;
//     for i in 1..n {
//         iter_num *= i;
//     }
//     now_factor_mul *= iter_num as f64;
//     for i in n..n + 10 {
//         // for i in n..n + 100_000_000 / 10000000 {
//         now_factor_mul *= factor_mul;
//         res += now_factor_mul * i as f64 * i as f64 * iter_base;
//         iter_base *= (i - 1) as f64;
//         if i > n - 2 {}
//         // let tmptmp_res = tmp_res * (i as f64) / (i - 1) as f64 * factor as f64 / factor_base as f64;
//         // res += tmptmp_res;
//         // tmp_res = tmptmp_res;
//         // println!("{}", tmptmp_res);
//         println!("{}", res);
//     }
//     println!("{}", res);
// }

// fn main() {
//     input! {
//         n: usize
//     }
//     let mut factor = 0.;
//     let mut iter_n = n as f64;
//     for i in 1..n {
//         factor *= i as f64 / iter_n;
//     }
//     let mut res = (iter_n - 1.) * factor;
//     let mut bef_tmp_res = res;
//     for _i in 0..100_000_000 {
//         let tmp_res = bef_tmp_res * ((iter_n + 1.) / iter_n) * factor;
//         res += tmp_res;
//         iter_n += 1.;
//         bef_tmp_res = tmp_res;
//     }
//     println!("{}", res);
// }
