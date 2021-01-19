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
        mut s: Chars
    }
    let mut black_cnt = 0;
    let mut white_cnt = 0;
    for i in 0..n {
        let s_i = s[i];
        if s_i == '.' {
            if black_cnt > 0 {
                white_cnt += 1;
            }
        } else {
            black_cnt += 1;
        }
    }
    println!("{}", min(black_cnt, white_cnt));
}


// fn main() {
//     input! {
//         n: usize,
//         mut s: Chars
//     }
//     let mut black_cnt = 0;
//     let mut res = 0;
//     for i in 0..n {
//         let s_i = s[i];
//         if s_i == '.' {
//             if black_cnt > 0 {
//                 res = min(res + 1, black_cnt);
//             }
//         } else {
//             black_cnt += 1;
//         }
//     }
//     println!("{}", res);
// }
