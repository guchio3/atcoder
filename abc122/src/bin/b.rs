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
        s: Chars
    }
    let acgt = vec!['A', 'C', 'G', 'T'];
    let mut res = 0;
    for start in 0..s.len() {
        'end_loop: for end in start+1..=s.len() {
            for i in start..end {
                if !acgt.contains(&s[i]) {
                    continue 'end_loop;
                }
            }
            res = max(res, end - start);
        }
    }
    println!("{}", res);
}


// fn main() {
//     input! {
//         s: Chars
//     }
//     let mut start = 0;
//     let mut end = 1;
//     let mut res = 0;
//     let acgt = vec!['A', 'C', 'G', 'T'];
//     while end <= s.len() {
//         if !acgt.contains(&s[end - 1]) {
//             res = max(res, end - start - 1);
//             start = end;
//         }
//         end += 1;
//     }
//     res = max(res, end - start - 1);
//     println!("{}", res);
// }
