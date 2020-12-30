#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut is_diff_sign = vec![false; n];
    let mut bef_a_i = 0;
    for i in 0..a.len() {
        let a_i = a[i];
        if a_i != 0 && bef_a_i != 0 && ((a_i > 0) != (bef_a_i > 0)) {
            is_diff_sign[i] = true;
        }
        bef_a_i = a_i;
    }
}
