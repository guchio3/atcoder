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
        n: usize, m: usize,
        mut ab: [(usize, usize); n]
    }
    ab.sort_by(|x, y| y.partial_cmp(&x).unwrap());

    let mut res = 0;
    let mut bh = BinaryHeap::new();
    for i in (0..m).rev() {
        while ab.len() > 0 && ab[ab.len() - 1].0 + i <= m {
            bh.push(ab.pop().unwrap().1);
        }
        if bh.len() > 0 {
            res += bh.pop().unwrap();
        }
    }
    println!("{}", res);
}

// fn main() {
//     input! {
//         n: usize, m: usize,
//         ab: [(usize, usize); n]
//     }
//     let mut ba: Vec<(usize, usize)> = ab.into_iter().map(|x| (x.1, x.0)).collect();
//     ba.sort();
//     let mut res = 0;
//     for day in 0..m {
//         while ba.len() > 0 && ba[ba.len() - 1].1 > (m - day) {
//             ba.pop();
//         }
//         if ba.len() > 0 {
//             res += ba.pop().unwrap().0;
//         }
//     }
//     println!("{}", res);
// }
