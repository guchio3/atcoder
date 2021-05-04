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
    }
    let mut lights = vec![vec![false; n]; m];
    for i in 0..m {
        input! {
            k_i: usize,
            s_i: [usize; k_i]
        }
        for s_i in s_i {
            lights[i][s_i - 1] = true;
        }
    }
    input! {
        p: [usize; m]
    }

    let mut res = 0;
    'outer: for i in 0..(1 << n) {
        let mut nums = vec![0; m];
        for j in 0..n {
            if i & (1 << j) > 0 {
                for k in 0..m {
                    if lights[k][j] {
                        nums[k] += 1;
                    }
                }
            }
        }
        for i in 0..m {
            if nums[i] % 2 != p[i] {
                continue 'outer;
            }
        }
        res += 1;
    }
    println!("{}", res);
}
