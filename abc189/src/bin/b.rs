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
        n: usize, mut x: usize,
        vp: [(usize, usize); n]
    }
    x *= 100;
    let mut cum = 0;
    for i in 0..n {
        let vp_i = vp[i];
        let v = vp_i.0;
        let p = vp_i.1;
        cum += v * p;
        if cum > x {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}

// fn main() {
//     input! {
//         n: usize, x: usize,
//         vp: [(usize, usize); n]
//     }
//     let mut cum: f64 = 0.;
//     for i in 0..n {
//         let vp_i = vp[i];
//         let v = vp_i.0;
//         let p = vp_i.1;
//         cum += (v * p) as f64 * 0.01;
//         if cum.ceil() as usize > x {
//             println!("{}", i + 1);
//             return;
//         }
//     }
//     println!("-1");
// }
