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
    let mut res: usize = 0;
    let mut cnt_dict = HashMap::new();
    for s_i in s {
        let mut element: Vec<char> = s_i.chars().collect();
        element.sort();
        if let Some(cnt) = cnt_dict.get(&element) {
            res += cnt;
        }
        (*cnt_dict.entry(element).or_insert(0)) += 1;
    }
    println!("{}", res);
}

// fn ncr(n: usize, mut r: usize) -> usize {
//     if r > n / 2 {
//         r = n - r;
//     }
//     let mut res = 1;
//     let start = max(n - r + 1, r);
//     let mut r_vec: Vec<usize> = (1..=r).into_iter().collect();
//     for i in start..=n {
//         res *= i;
//         let mut non_used = vec![];
//         for &r_vec_i in r_vec.iter() {
//             if res % r_vec_i == 0 {
//                 res /= r_vec_i;
//             } else {
//                 non_used.push(r_vec_i);
//             }
//         }
//         r_vec = non_used;
//     }
//     res
// }
//
// fn main() {
//     input! {
//         n: usize,
//         s: [String; n]
//     }
//     let mut cnt_dict = HashMap::new();
//     for s_i in s {
//         let mut element: Vec<char> = s_i.chars().collect();
//         element.sort();
//         (*cnt_dict.entry(element).or_insert(0)) += 1;
//     }
//     let mut res: usize = 0;
//     for (_element, cnt) in cnt_dict {
//         if cnt > 1 {
//             res += ncr(cnt, 2);
//         }
//     }
//     println!("{}", res);
// }
