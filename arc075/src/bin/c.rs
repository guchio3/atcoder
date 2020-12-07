#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        mut s: [i64; n]
    }
    s.sort();

    let mut min_ten_s_i = -1;
    let mut sum = 0;
    for i in 0..s.len() {
        let s_i = s[i];
        if s_i % 10 != 0 && min_ten_s_i == -1 {
            min_ten_s_i = s[i];
        }
        sum += s_i;
    }

    if sum % 10 == 0 {
        if min_ten_s_i >= 0 {
            sum -= min_ten_s_i;
        } else {
            sum = 0;
        }
    } else {
        println!("{}", sum);
    }
}
