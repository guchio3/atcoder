#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        s: Chars,
    }
    let mut res = 0;
    for i in 0..s.len() {
        let s_i = s[i];
        if s_i == 'U' {
            res += s.len() - i - 1;
            res += 2 * i;
        } else {
            res += 2 * (s.len() - i - 1);
            res += i;
        }
    }
    println!("{}", res);
}
