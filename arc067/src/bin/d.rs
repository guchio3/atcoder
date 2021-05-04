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
        n: usize, a: i64, b: i64,
        x: [i64; n]
    }
    let mut bef_x_i = x[0];
    let mut res = 0;
    for i in 0..n {
        let x_i = x[i];
        if (x_i - bef_x_i) * a > b {
            res += b;
        } else {
            res += (x_i - bef_x_i) * a;
        }
        bef_x_i = x_i;
    }
    println!("{}", res);
}
