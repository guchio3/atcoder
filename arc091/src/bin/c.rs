#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize, m: usize
    }
    let nn;
    let mm;
    if n == 1 {
        nn = 1;
    } else {
        nn = n - 2;
    }
    if m == 1 {
        mm = 1;
    } else {
        mm = m - 2;
    }

    let res = nn * mm;
    println!("{}", res);
}
