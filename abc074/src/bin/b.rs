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
        k: usize,
        x: [usize; n]
    }
    let mut res = 0;
    for i in 0..n {
        let x_i = x[i];
        if (k - x_i) > x_i {
            res += 2 * x_i;
        } else {
            res += 2 * (k - x_i);
        }
    }
    println!("{}", res);
}
