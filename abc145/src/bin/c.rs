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
        xy: [(f64, f64); n]
    }
    // let mut all_distance = 0.;
    // // for i in 0..n {
    // //     all_distance += ((xy[i].0 - xy[(i + 1) % n].0).powi(2)
    // //         + (xy[i].1 - xy[(i + 1) % n].1).powi(2))
    // //     .powf(0.5)
    // // }
    // for i in 0..n {
    //     for j in 0..n {
    //         if i == j {
    //             continue;
    //         }
    //         all_distance += ((xy[i].0 - xy[j].0).powi(2) + (xy[i].1 - xy[j].1).powi(2)).powf(0.5)
    //     }
    // }
    // println!("{}", all_distance * (n - 1) as f64 / n as f64);

    let mut all = 0.;
    let mut cnt = 0;
    for order_i in (0..n).permutations(n) {
        let mut distance = 0.;
        let mut bef_j = order_i[0];
        for j_idx in 1..n {
            let j = order_i[j_idx];
            distance +=
                ((xy[bef_j].0 - xy[j].0).powi(2) + (xy[bef_j].1 - xy[j].1).powi(2)).powf(0.5);
            bef_j = j;
        }
        all += distance;
        cnt += 1;
    }
    println!("{}", all / cnt as f64);
}
