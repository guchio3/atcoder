#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use ordered_float::NotNan;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q]
    }
    let mut one_cumsum = vec![0; n + 1];
    let mut two_cumsum = vec![0; n + 1];
    for i in 0..n {
        let (c_i, p_i) = cp[i];
        match c_i {
            1 => {
                one_cumsum[i + 1] = one_cumsum[i] + p_i;
                two_cumsum[i + 1] = two_cumsum[i];
            }
            2 => {
                one_cumsum[i + 1] = one_cumsum[i];
                two_cumsum[i + 1] = two_cumsum[i] + p_i;
            }
            _ => {}
        }
    }

    for (l_i, r_i) in lr {
        println!(
            "{} {}",
            one_cumsum[r_i] - one_cumsum[l_i - 1],
            two_cumsum[r_i] - two_cumsum[l_i - 1]
        );
    }
}
