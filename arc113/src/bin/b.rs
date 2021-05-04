#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn mod_pow(mut x: usize, mut n: usize, mod_num: usize) -> usize {
    let mut res = 1;
    while n > 0 {
        if n & 1 > 0 {
            res = (res * x) % mod_num;
        }
        x = (x * x) % mod_num;
        n >>= 1;
    }
    res
}

fn main() {
    input! {
        a: usize, b: usize, c: usize
    }
    let mod_res = mod_pow(b, c, 4);
    let mut res = a % 10;
    for _i in 0..((mod_res + 4 - 1) % 4) {
        res = (res * (a % 10)) % 10;
    }
    println!("{}", res);
}

// fn main() {
//     input! {
//         a: usize, mut b: usize, c: usize
//     }
//     let mut cnt = 1;
//     let mut cnts = vec![0; 10];
//     let a_base = a % 10;
//     let mut num_base = a_base;
//     loop {
//         if cnts[num_base] != 0 {
//             break;
//         } else {
//             cnts[num_base] = cnt;
//         }
//         num_base = (num_base * a_base) % 10;
//         cnt += 1;
//     }
//     let a_period = cnt - cnts[num_base];
//
//     let b_rest = b % a_period;
//     let mut cnt = 1;
//     let mut cnts = vec![0; a_period];
//     let b_rest_base = b_rest;
//     let mut num_base = b_rest_base;
//     loop {
//         if cnts[num_base] != 0 {
//             break;
//         } else {
//             cnts[num_base] = cnt;
//         }
//         num_base = (num_base * b_rest_base) % a_period;
//         cnt += 1;
//     }
//     let b_rest_period = cnt - cnts[num_base];
//
//     let b_num = b % a_period;
//     let mut num_base = b_num;
//     for _i in 0..((c - 1) % b_rest_period) {
//         num_base = (num_base * b_num) % a_period;
//     }
//
//     let a_num = a % 10;
//     let mut res = a_num;
//     for _i in 0..(max(1, num_base) - 1) {
//         res = (res * a_num) % 10;
//     }
//     println!("{}", res);
// }

// fn main() {
//     input! {
//         a: usize, mut b: usize, c: usize
//     }
//     let mut cnt = 1;
//     let mut cnts = vec![0; 10];
//     let a_base = a % 10;
//     let mut num_base = a_base;
//     loop {
//         if cnts[num_base] != 0 {
//             break;
//         } else {
//             cnts[num_base] = cnt;
//         }
//         num_base = (num_base * a_base) % 10;
//         cnt += 1;
//     }
//     let a_first_step = cnts[num_base] - 1;
//     let a_period = cnt - cnts[num_base];
//     if a_first_step > b {
//         let mut res = a % 10;
//         for _i in 0..a_first_step {
//             res = (res * (a % 10)) % 10;
//         }
//         println!("{}", res);
//         return;
//     } else {
//         b -= a_first_step;
//     }
//
//     let b_rest = b % a_period;
//     let mut cnt = 1;
//     let mut cnts = vec![0; a_period];
//     let b_rest_base = b_rest;
//     let mut num_base = b_rest_base;
//     loop {
//         if cnts[num_base] != 0 {
//             break;
//         } else {
//             cnts[num_base] = cnt;
//         }
//         num_base = (num_base * b_rest_base) % a_period;
//         cnt += 1;
//     }
//     let b_first_step = cnts[num_base] - 1;
//     let b_rest_period = cnt - cnts[num_base];
//
//     let b_num = b % a_period;
//     let mut num_base = b_num;
//     for _i in 0..min(c - 1, b_first_step) {
//         num_base = (num_base * b_num) % a_period;
//     }
//     for _i in 0..((c - 1 - min(c - 1, b_first_step)) % b_rest_period) {
//         num_base = (num_base * b_num) % a_period;
//     }
//
//     let a_num = a % 10;
//     let mut res = a_num;
//     // println!("a_first_step: {}", a_first_step);
//     // println!("b: {}", b);
//     // println!("a_period: {}", a_period);
//     // for _i in 0..min(a_first_step + b_first_step + num_base, b - 1) {
//     //     res = (res * a_num) % 10;
//     // }
//     // for _i in 0..(b - 1 - min(a_first_step, b - 1)) % a_period {
//     //     res = (res * a_num) % 10;
//     // }
//     if a_first_step + num_base > 0 {
//         for _i in 0..(a_first_step + num_base - 1) {
//             res = (res * a_num) % 10;
//         }
//     }
//     println!("{}", res);
// }
