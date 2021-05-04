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
        mut n: usize, k: usize,
        d: [usize; k]
    }
    let mut usable_nums = vec![];
    for i in 0..10 {
        if !d.contains(&i) {
            usable_nums.push(i);
        }
    }
    'outer: loop {
        let mut n_copy = n;
        while n_copy > 0 {
            if !usable_nums.contains(&(n_copy % 10)) {
                n += 1;
                continue 'outer;
            }
            n_copy /= 10;
        }
        println!("{}", n);
        return;
    }
}

// fn main() {
//     input! {
//         mut n: usize, k: usize,
//         d: [usize; k]
//     }
//     let mut usable_nums = vec![];
//     for i in 0..10 {
//         if !d.contains(&i) {
//             usable_nums.push(i);
//         }
//     }
//     let min_usable_num = usable_nums[0];
//     let second_min_usable_num;
//     if usable_nums.len() > 1 {
//         second_min_usable_num = usable_nums[1];
//     } else {
//         second_min_usable_num = usable_nums[0];
//     }
//
//     let mut digit_nums = vec![];
//     while n > 0 {
//         digit_nums.push(n % 10);
//         n /= 10;
//     }
//     digit_nums = digit_nums.into_iter().rev().collect();
//
//     let mut next_res = vec![];
//     if min_usable_num == 0 {
//         next_res.push(second_min_usable_num);
//     } else {
//         next_res.push(min_usable_num);
//     }
//     for _i in 0..digit_nums.len() {
//         next_res.push(min_usable_num);
//     }
//
//     let mut reses = vec![0; digit_nums.len()];
//     'outer: for i in 0..digit_nums.len() {
//         let digit_num = digit_nums[i];
//         for j in 0..usable_nums.len() {
//             let usable_num = usable_nums[j];
//             if usable_num == digit_num {
//                 reses[i] = usable_num;
//                 continue 'outer;
//             } else if usable_num > digit_num {
//                 reses[i] = usable_num;
//                 for l in i + 1..digit_nums.len() {
//                     reses[l] = min_usable_num;
//                 }
//                 break 'outer;
//             }
//         }
//         for j in (0..i).rev() {
//             let current_num = reses[j];
//             for k in 0..usable_nums.len() {
//                 let usable_num = usable_nums[k];
//                 if usable_num > current_num {
//                     reses[j] = usable_num;
//                     for l in j + 1..digit_nums.len() {
//                         reses[l] = min_usable_num;
//                     }
//                     break 'outer;
//                 }
//             }
//         }
//         reses = next_res;
//         break;
//     }
//
//     println!("{}", reses.into_iter().join(""));
// }
