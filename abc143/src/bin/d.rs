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
        mut l: [usize; n]
    }
    l.sort();
    let mut res: usize = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if l[i] + l[j] > l[k] {
                    res += 1;
                }
            }
        }
    }
    println!("{}", res);
}

// fn main() {
//     input! {
//         n: usize,
//         mut l: [usize; n]
//     }
//     l.sort();
//     let mut res: usize = 0;
//     for i in 0..n {
//         for j in i + 1..n {
//             let index = l.binary_search_by(|x| x.cmp(&(l[i] + l[j] - 1)));
//             let index_num = match index {
//                 Ok(v) => v + 1,
//                 Err(v) => v,
//             };
//             if index_num > j + 1 {
//                 res += index_num - j - 1;
//             }
//         }
//     }
//     println!("{}", res);
// }
