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
        mut a: [i64; n]
    }
    // 各ペア 1 度なので sort すれば良い
    a.sort();
    let mut res = 0;
    let mut cumsum = a[0];
    for i in 1..a.len() {
        let a_i = a[i] as i64;
        res += a[i] * i as i64 - cumsum;
        cumsum += a_i;
    }

    println!("{}", res);
}
