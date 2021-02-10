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
        h: usize, w: usize,
        mut s: [Chars; h]
    }
    let mut res = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let mut black_num = 0;
            black_num += (s[i][j] == '#') as usize;
            black_num += (s[i + 1][j] == '#') as usize;
            black_num += (s[i][j + 1] == '#') as usize;
            black_num += (s[i + 1][j + 1] == '#') as usize;
            if black_num == 1 || black_num == 3 {
                res += 1;
            }
        }
    }
    println!("{}", res);
}

// fn main() {
//     input! {
//         h: usize, w: usize,
//         mut s: [Chars; h]
//     }
//     let mut res = 0;
//     for i in 0..h {
//         for j in 0..w {
//             if i > 0 && i < h - 1 && j > 0 && j < w - 1 {
//                 if s[i][j] == '#' {
//                     if s[i - 1][j] == '#' && s[i + 1][j] == '#' {
//                         continue;
//                     }
//                     if s[i][j - 1] == '#' && s[i][j + 1] == '#' {
//                         continue;
//                     }
//                     if s[i - 1][j - 1] == '#' && s[i + 1][j + 1] == '#' {
//                         continue;
//                     }
//                     if s[i + 1][j - 1] == '#' && s[i - 1][j + 1] == '#' {
//                         continue;
//                     }
//                     res += 1;
//                 }
//             }
//         }
//     }
//     if res == 1 {
//         println!("4");
//     } else {
//         println!("{}", res);
//     }
// }

// fn main() {
//     input! {
//         h: usize, w: usize,
//         mut s: [Chars; h]
//     }
//     let mut res = 0;
//     let mut minus = 0;
//     for i in 0..h {
//         for j in 0..w {
//             if i > 0 && i < h - 1 && j > 0 && j < w - 1 {
//                 if s[i][j] == '#' {
//                     res += 1;
//                     let mut black_num = 0;
//                     if s[i-1][j] == '#' {
//                         black_num += 1;
//                     }
//                     if s[i+1][j] == '#' {
//                         black_num += 1;
//                     }
//                     if s[i][j-1] == '#' {
//                         black_num += 1;
//                     }
//                     if s[i][j+1] == '#' {
//                         black_num += 1;
//                     }
//                     if black_num >= 3 {
//                         minus += 1;
//                     }
//                 }
//             }
//         }
//     }
//     println!("{}", res - minus);
// }
