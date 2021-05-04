#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize
    }
}

// fn main() {
//     input! {
//         n: usize
//     }
//     let mut res: usize = 0;
//     let mut num = 1;
//     while num <= n {
//         let div = n / num;
//         if (n - div) % div == 0 {
//             let y = (n - div) / div;
//             if num <= y && y <= n / div {
//                 res += (n - div) / div;
//             }
//             // println!("num: {} / div: {} / res: {}", num, div, res);
//         }
//         num = n / div + 1;
//     }
//     println!("{}", res);
// }
