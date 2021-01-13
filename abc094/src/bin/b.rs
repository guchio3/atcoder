#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        _n: usize, m: usize, x: usize,
        a: [usize; m]
    }
    let mut tmp = 0;
    let mut tmp2 = 0;
    for a_i in a {
        if a_i > x {
            tmp2 += 1;
        } else if a_i < x {
            tmp += 1;
        }
    }
    println!("{}", min(tmp, tmp2));
}
