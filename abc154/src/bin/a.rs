#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        s: String, t: String,
        mut a: usize, mut b: usize,
        u: String
    }
    if s == u {
        a -= 1;
    } else {
        b -= 1;
    }
    println!("{} {}", a, b);
}
