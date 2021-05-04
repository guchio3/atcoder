#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn check(m: u128, d: u128, x: &Vec<u128>) -> bool {
    let mut cumed: u128 = 0;
    let mut digit: u128 = 1;
    for i in 0..x.len() {
        if digit > m {
            return true;
        }
        cumed += digit * x[x.len() - i - 1];
        if cumed > m {
            break;
        }
        digit *= d;
    }
    cumed > m
}

fn main() {
    input! {
        x: Chars,
        m: u128
    }
    let x: Vec<u128> = x.into_iter().map(|xx| xx as u128 - '0' as u128).collect();
    if x.len() == 1 {
        println!("{}", (x[0] <= m) as u128);
    } else {
        let d = x.iter().map(|xx| *xx).max().unwrap();
        let res;
        if m + 1 < d {
            res = 0;
        } else {
            if check(m, d + 1, &x) {
                res = 0;
            } else {
                let mut left = d;
                let mut right = m + 1;
                while right - left > 1 {
                    if check(m, (right + left) / 2, &x) {
                        right = (right + left) / 2;
                    } else {
                        left = (right + left) / 2;
                    }
                }
                res = left - d;
            }
        }
        println!("{}", res);
    }
}

// fn check(m: usize, d: usize, x: &Vec<usize>) -> bool {
//     let mut cumed: usize = 0;
//     let mut digit: usize = 1;
//     for i in 0..x.len() {
//         if let Some(digit_x) = digit.checked_mul(x[x.len() - i - 1]) {
//             if let Some(next_cumed) = cumed.checked_add(digit_x) {
//                 cumed = next_cumed;
//             } else {
//                 return true;
//             }
//         } else {
//             return true;
//         }
//         if let Some(next_digit) = digit.checked_mul(d) {
//             digit = next_digit;
//         } else {
//             return true;
//         }
//     }
//     cumed > m
// }
//
// fn main() {
//     input! {
//         x: Chars,
//         m: usize
//     }
//     let x: Vec<usize> = x.into_iter().map(|xx| xx as usize - '0' as usize).collect();
//     if x.len() == 1 {
//         println!("{}", (x[0] <= m) as usize);
//     } else {
//         let d = x.iter().map(|xx| *xx).max().unwrap();
//         let res;
//         if m < d {
//             res = 0;
//         } else {
//             if check(m, d + 1, &x) {
//                 res = 0;
//             } else {
//                 let mut left = d + 1;
//                 let mut right = m + 1;
//                 while right - left > 1 {
//                     if check(m, (right + left) / 2, &x) {
//                         right = (right + left) / 2;
//                     } else {
//                         left = (right + left) / 2;
//                     }
//                 }
//                 res = left - d;
//             }
//         }
//         println!("{}", res);
//     }
// }
