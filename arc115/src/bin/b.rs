#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

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

fn main() {
    input! {
        n: usize,
        c: [[i64; n]; n]
    }
    let mut row_sums = vec![];
    let mut row_diffs = vec![];
    let mut min_row_diff = 0;
    for i in 0..n {
        let mut row_sum = 0;
        for j in 0..n {
            row_sum += c[i][j];
        }
        row_sums.push(row_sum);
        let diff = row_sum - row_sums[0];
        if diff % n as i64 != 0 {
            println!("No");
            return;
        } else {
            let row_diff = diff / n as i64;
            row_diffs.push(row_diff);
            min_row_diff = min(min_row_diff, row_diff)
        }
    }
    for i in 0..n {
        row_diffs[i] -= min_row_diff;
    }
    let a: Vec<i64> = row_diffs.iter().map(|x| *x).collect();
    let mut b = vec![];
    for i in 0..n {
        let b_i = c[0][i] - a[0];
        if b_i < 0 {
            println!("No");
            return;
        } else {
            b.push(b_i);
        }
    }

    for i in 0..n {
        for j in 0..n {
            if c[i][j] != a[i] + b[j] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
    println!("{}", a.into_iter().join(" "));
    println!("{}", b.into_iter().join(" "));
}

// fn main() {
//     input! {
//         n: usize,
//         c: [[i64; n]; n]
//     }
//     let mut row_sums = vec![];
//     for i in 0..n {
//         let mut row_sum = 0;
//         for j in 0..n {
//             row_sum += c[i][j];
//         }
//         row_sums.push(row_sum);
//     }
//
//     let mut gcd_res = row_sums[0];
//     for &row_sum in row_sums.iter() {
//         gcd_res = gcd(gcd_res, row_sum);
//     }
//
//     let divisors = get_divisors(gcd_res);
//
//     'outer: for divisor in divisors {
//         if divisor < n as i64 {
//             continue;
//         }
//         let mut a = vec![];
//         for &row_sum in row_sums.iter() {
//             a.push(row_sum / divisor);
//             println!("row_sum: {} / divisor: {}", row_sum, divisor);
//         }
//         println!("{:?}", a);
//         let mut b = vec![];
//         for i in 0..n {
//             let b_i = c[0][i] - a[0];
//             if b_i < 0 {
//                 continue 'outer;
//             }
//             b.push(b_i);
//         }
//         println!("{}", a.into_iter().join(" "));
//         println!("{}", b.into_iter().join(" "));
//         return;
//     }
//
//     println!("No");
// }
