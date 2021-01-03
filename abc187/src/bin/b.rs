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
    let mut res = 0;
    for i in 0..n {
        for j in i+1..n {
            let xy_i = xy[i];
            let xy_j = xy[j];
            // println!("{}, {}, {}", i, j, ((xy_i.1 - xy_j.1) / (xy_i.0 - xy_j.0)).abs());
            if ((xy_i.1 - xy_j.1) / (xy_i.0 - xy_j.0)).abs() <= 1. {
                res += 1;
            }
        }
    }
    println!("{}", res );
}
