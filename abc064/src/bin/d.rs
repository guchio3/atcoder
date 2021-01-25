#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        _n: usize, s: Chars
    }
    let mut left_num = 0;
    let mut res = VecDeque::new();
    for s_i in s {
        if s_i == '(' {
            res.push_back(s_i);
            left_num += 1;
        } else {
            res.push_back(s_i);
            if left_num == 0 {
                res.push_front('(');
            } else {
                left_num -= 1;
            }
        }
    }
    while left_num > 0 {
        res.push_back(')');
        left_num -= 1;
    }
    println!("{}", res.into_iter().join(""));
}
