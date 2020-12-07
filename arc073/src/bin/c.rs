#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize, T: usize,
        t: [usize; n]
    }

    let mut res = 0;
    let mut bef_t_i = 0;
    for t_i in t {
        let diff = t_i - bef_t_i;
        res += min(T, diff);
        bef_t_i = t_i;
    }
    res += T;

    println!("{}", res);
}
