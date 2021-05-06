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
        h: usize, w: usize,
        a: [[usize; w]; h]
    }
    let mut row_sums = vec![];
    for i in 0..h {
        row_sums.push(a[i].iter().sum::<usize>());
    }
    let mut col_sums = vec![];
    for i in 0..w {
        let mut val = 0;
        for j in 0..h {
            val += a[j][i];
        }
        col_sums.push(val);
    }

    for i in 0..h {
        let mut res = vec![];
        for j in 0..w {
            res.push(row_sums[i] + col_sums[j] - a[i][j]);
        }
        println!("{}", res.into_iter().join(" "));
    }
}
