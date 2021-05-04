#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut ls = vec![0, a[0]];
    for i in 2..n {
        ls.push(gcd(a[i - 1], ls[ls.len() - 1]));
    }
    let mut rs = vec![0, a[n - 1]];
    for i in (0..n - 2).rev() {
        rs.push(gcd(a[i + 1], rs[rs.len() - 1]));
    }
    rs = rs.into_iter().rev().collect();

    let mut res = max(ls[n - 1], rs[0]);
    for i in 1..n - 1 {
        res = max(res, gcd(ls[i], rs[i]));
    }

    println!("{}", res);
}

// fn get_divisors<T>(num: T) -> Vec<T>
// where
//     T: PrimInt + NumAssign,
// {
//     let mut i = T::one();
//     let mut res = vec![];
//     while i * i <= num {
//         if num % i == T::zero() {
//             let rev_i = num / i;
//             res.push(i);
//             if i != rev_i {
//                 res.push(rev_i);
//             }
//         }
//         i += T::one();
//     }
//     res
// }
//
// fn main() {
//     input! {
//         n: usize,
//         a: [usize; n]
//     }
//     let divisors_0 = get_divisors(a[0]);
//     let divisors_1 = get_divisors(a[1]);
//
//     let mut cnts = HashMap::new();
//     for i in divisors_0 {
//         *cnts.entry(i).or_insert(0) += 1;
//     }
//     for i in divisors_1 {
//         *cnts.entry(i).or_insert(0) += 1;
//     }
//
//     let keys: Vec<usize> = cnts.keys().map(|x| *x).collect();
//
//     for i in 2..n {
//         let a_i = a[i];
//         for &key in keys.iter() {
//             if a_i % key == 0 {
//                 cnts.entry(key).and_modify(|e| *e += 1);
//             }
//         }
//     }
//
//     let mut res = 0;
//     for (key, value) in cnts {
//         if value >= n - 1 {
//             res = max(res, key);
//         }
//     }
//     println!("{}", res);
// }
