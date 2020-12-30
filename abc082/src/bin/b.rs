#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars
    }
    s.sort();
    t.sort();
    t = t.into_iter().rev().collect();
    if s < t {
        println!("Yes");
    } else {
        println!("No");
    }
}
