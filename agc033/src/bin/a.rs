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
        mut a: [Chars; h]
    }
    let mut op_nums = vec![vec![1_000_000; w]; h];
    let mut deque = VecDeque::new();

    let mut target_points = vec![];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                op_nums[i][j] = 0;
                deque.push_back((i, j));
            } else {
                target_points.push((i, j));
            }
        }
    }

    while let Some((i, j)) = deque.pop_front() {
        if i > 0 && a[i - 1][j] == '.' {
            op_nums[i - 1][j] = op_nums[i][j] + 1;
            a[i - 1][j] = '#';
            deque.push_back((i - 1, j));
        }
        if j > 0 && a[i][j - 1] == '.' {
            op_nums[i][j - 1] = op_nums[i][j] + 1;
            a[i][j - 1] = '#';
            deque.push_back((i, j - 1));
        }
        if h - 1 > i && a[i + 1][j] == '.' {
            op_nums[i + 1][j] = op_nums[i][j] + 1;
            a[i + 1][j] = '#';
            deque.push_back((i + 1, j));
        }
        if w - 1 > j && a[i][j + 1] == '.' {
            op_nums[i][j + 1] = op_nums[i][j] + 1;
            a[i][j + 1] = '#';
            deque.push_back((i, j + 1));
        }
    }

    let mut res = 0;
    for (i, j) in target_points {
        res = max(res, op_nums[i][j]);
    }

    println!("{}", res);
}

// fn main() {
//     input! {
//         h: usize, w: usize,
//         a: [Chars; h]
//     }
//     let mut op_nums = vec![vec![1_000_000; w]; h];
//
//     for i in 0..h {
//         let mut bef_black_j = 1_000_000;
//         for j in 0..w {
//             if a[i][j] == '.' {
//                 if bef_black_j != 1_000_000 {
//                     op_nums[i][j] = min(op_nums[i][j], j - bef_black_j);
//                 }
//             } else {
//                 if bef_black_j == 1_000_000 {
//                     bef_black_j = 0;
//                 }
//                 for k in bef_black_j..j {
//                     if a[i][j] == '.' {
//                         op_nums[i][j] = min(op_nums[i][j], j - k);
//                     }
//                 }
//                 bef_black_j = j;
//             }
//             println!("{:?}", op_nums);
//         }
//     }
//
//     for j in 0..w {
//         let mut bef_black_i = 1_000_000;
//         for i in 0..h {
//             if a[i][j] == '.' {
//                 if bef_black_i != 1_000_000 {
//                     op_nums[i][j] = min(op_nums[i][j], i - bef_black_i);
//                 }
//             } else {
//                 if bef_black_i == 1_000_000 {
//                     bef_black_i = 0;
//                 }
//                 for k in bef_black_i..i {
//                     if a[i][j] == '.' {
//                         op_nums[i][j] = min(op_nums[i][j], i - k);
//                     }
//                 }
//                 bef_black_i = i;
//             }
//         }
//     }
//     println!("{:?}", op_nums);
//
//     let mut res = 0;
//     for i in 0..h {
//         for j in 0..w {
//             if a[i][j] == '.' {
//                 res = max(res, op_nums[i][j]);
//             }
//         }
//     }
//     println!("{}", res);
// }
