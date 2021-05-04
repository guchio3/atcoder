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
        a: [i64; n]
    }
    let mut diffs = vec![0; n];
    for i in 0..n {
        diffs[i] = a[i] - i as i64 - 1;
    }
    // diffs.sort();
    // let b = diffs[diffs.len() / 2];
    // let mut res = 0;
    // for i in 0..n {
    //     res += (diffs[i] - b).abs();
    // }
    // println!("{}", res);

    let mut l = -1_000_000_000;
    let mut r = 1_000_000_000;
    loop {
        let mid = (r + l) / 2;
        let mut mid_res = 0;
        let mut left_res = 0;
        let mut right_res = 0;
        for i in 0..n {
            mid_res += (a[i] - i as i64 - 1 - mid).abs();
            left_res += (a[i] - i as i64 - 1 - mid - 1).abs();
            right_res += (a[i] - i as i64 - 1 - mid + 1).abs();
        }
        // println!("mid: {}", mid);
        // println!(
        //     "mid_res: {} / left_res: {} / right_res:  {}",
        //     mid_res, left_res, right_res
        // );
        if left_res >= mid_res && right_res >= mid_res {
            println!("{}", mid_res);
            return;
        } else if left_res < mid_res {
            l = mid;
        } else {
            r = mid;
        }
    }
}

// fn main() {
//     input! {
//         n: usize,
//         a: [i64; n]
//     }
//     let mut diffs = vec![0; n];
//     // let mut max_diff = 1_000_000_000;
//     // let mut min_diff = -1_000_000_000;
//     for i in 0..n {
//         diffs[i] = a[i] - i as i64 - 1;
//         // max_diff = max(max_diff, diffs[i]);
//         // min_diff = min(min_diff, diffs[i]);
//     }
//     diffs.sort();
//     let b = diffs[diffs.len() / 2];
//     let mut res = 0;
//     for i in 0..n {
//         res += (diffs[i] - b).abs();
//     }
//     println!("{}", res);
//
//     // println!("max_diff {}", max_diff);
//     // println!("min_diff {}", min_diff);
//     // let mut l = min_diff;
//     // let mut r = max_diff;
//     // let mut l = -1_000_000_000;
//     // let mut r = 1_000_000_000;
//     // for _i in 0..10 {
//     //     let mid = (r - l) / 2;
//     //     let mut mid_res = 0;
//     //     let mut left_res = 0;
//     //     let mut right_res = 0;
//     //     for i in 0..n {
//     //         mid_res += (a[i] - i as i64 - 1 - mid).abs();
//     //         left_res += (a[i] - i as i64 - 1 - mid - 1).abs();
//     //         right_res += (a[i] - i as i64 - 1 - mid + 1).abs();
//     //     }
//     //     println!("{}", mid);
//     //     println!("{} {} {}", mid_res, left_res, right_res);
//     //     if left_res >= mid_res && right_res >= mid_res {
//     //         println!("{}", mid);
//     //     } else if left_res < mid_res {
//     //         r = mid;
//     //     } else {
//     //         l = mid;
//     //     }
//     // }
// }
