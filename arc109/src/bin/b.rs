#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: i64
    }
    let mut minus = 0;
    let mut cnt = 0;
    while minus <= (n + 1) {
        cnt += 1;
        minus += cnt;
    }
    cnt -= 1;
    println!("{}", n - cnt + 1);
}
