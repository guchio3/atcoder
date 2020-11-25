#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize, mut x: i64, s: Chars
    }

    for s_i in s {
        if s_i == 'o' {
            x += 1;
        } else if s_i == 'x' {
            x = max(0, x - 1);
        }
    }

    println!("{}", x);
}
