#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        s: Chars,
        q: usize,
        tab: [(usize, usize, usize); q]
    }
    let mut left_s = vec![];
    let mut right_s = vec![];
    let mut switch = 0;
    for i in 0..n {
        left_s.push(s[i]);
        right_s.push(s[n + i]);
    }
    for (t_i, a_i, b_i) in tab {
        if t_i == 1 {
            if a_i <= n && b_i <= n {
                if switch == 0 {
                    let tmp = left_s[a_i - 1];
                    left_s[a_i - 1] = left_s[b_i - 1];
                    left_s[b_i - 1] = tmp;
                } else {
                    let tmp = right_s[a_i - 1];
                    right_s[a_i - 1] = right_s[b_i - 1];
                    right_s[b_i - 1] = tmp;
                }
            } else if a_i > n && b_i > n {
                if switch == 0 {
                    let tmp = right_s[a_i - n - 1];
                    right_s[a_i - n - 1] = right_s[b_i - n - 1];
                    right_s[b_i - n - 1] = tmp;
                } else {
                    let tmp = left_s[a_i - n - 1];
                    left_s[a_i - n - 1] = left_s[b_i - n - 1];
                    left_s[b_i - n - 1] = tmp;
                }
            } else {
                if switch == 0 {
                    let tmp = left_s[a_i - 1];
                    left_s[a_i - 1] = right_s[b_i - n - 1];
                    right_s[b_i - n - 1] = tmp;
                } else {
                    let tmp = right_s[a_i - 1];
                    right_s[a_i - 1] = left_s[b_i - n - 1];
                    left_s[b_i - n - 1] = tmp;
                }
            }
        } else {
            switch = (switch + 1) % 2;
        }
    }
    let mut res = vec![];
    if switch == 0 {
        for i in 0..n {
            res.push(left_s[i]);
        }
        for i in 0..n {
            res.push(right_s[i]);
        }
    } else {
        for i in 0..n {
            res.push(right_s[i]);
        }
        for i in 0..n {
            res.push(left_s[i]);
        }
    }
    println!("{}", res.into_iter().join(""));
}
