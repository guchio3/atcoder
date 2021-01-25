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
        n: usize, k: usize,
    }
    let mut res = 0;
    if k <= n {
        let n_k = (n - k) / k + 1;
        if n_k >= 2 {
            res += n_k * (n_k - 1) * (n_k - 2);
        } else {
            res += 0;
        }
        if n_k >= 1 {
            res += n_k * (n_k - 1) * 3;
        } else {
            res += 0;
        }
        res += n_k;
    }
    if k % 2 == 0 && k / 2 <= n {
        let n_k = (n - k / 2) / k + 1;
        if n_k >= 2 {
            res += n_k * (n_k - 1) * (n_k - 2);
        } else {
            res += 0;
        }
        if n_k >= 1 {
            res += n_k * (n_k - 1) * 3;
        } else {
            res += 0;
        }
        res += n_k;
    }
    println!("{}", res);
}
