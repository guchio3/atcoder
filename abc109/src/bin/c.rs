#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

// fn get_prime_factors<T>(num: T) -> Vec<T>
// where
//     T: Num + Integer + NumAssign,
// {
//     let mut res = vec![];
//     let mut iter_num = num;
//     for i in T::one()..num {
//         if i * i > num {
//             break;
//         }
//         while iter_num % i == T::zero() {
//             res.push(i);
//             iter_num /= i;
//         }
//     }
//     res
// }

fn get_prime_factors(num: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut iter_num = num;
    for i in 2..=num {
        if i > iter_num {
            break;
        }
        while iter_num % i == 0 {
            res.push(i);
            iter_num /= i;
        }
    }
    res
}

fn main() {
    input! {
        n: usize, xxx: i64,
        x: [i64; n]
    }
    let mut diffs = vec![];
    for x_i in x {
        diffs.push((x_i - xxx).abs());
    }
    let min_diff = *diffs.iter().min().unwrap();
    let mut prime_factors = get_prime_factors(min_diff);
    for mut diff in diffs {
        let mut tmp_prime_factors = vec![];
        for i in 0..prime_factors.len() {
            let prime_factor = prime_factors[i];
            if diff % prime_factor == 0 {
                diff /= prime_factor;
                tmp_prime_factors.push(prime_factor);
            }
        }
        prime_factors = tmp_prime_factors;
    }
    let res = prime_factors.iter().fold(1, |prod, x| prod * x);
    println!("{}", res);
}

// fn main() {
//     input! {
//         n: usize, xxx: i64,
//         x: [i64; n]
//     }
//     let mut diffs = vec![];
//     for x_i in x {
//         diffs.push((x_i - xxx).abs());
//     }
//     let mut res = diffs[0];
//     for i in 1..n {
//         res = gcd(res, diffs[i]);
//     }
//     println!("{}", res);
// }
