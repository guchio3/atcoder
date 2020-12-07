#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        n: usize,
    }

    let mut res = 0;
    for i in 0..=n / r {
        for j in 0..=(n - r * i) / g {
            let tmp = n - r * i - g * j;
            if tmp % b == 0 {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
