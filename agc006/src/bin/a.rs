#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars
    }

    let mut res = n * 2;
    'outer: for i in 0..n {
        for j in 0..(n - i) {
            if s[i + j] != t[j] {
                continue 'outer;
            }
        }
        res = n + i;
        break;
    }

    println!("{}", res);
}
