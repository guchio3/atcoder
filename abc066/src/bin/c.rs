#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: Chars
    }
    let mut res_deque = VecDeque::new();
    for i in 0..n {
        if i % 2 == 0 {
            res_deque.push_back(a[i]);
        } else {
            res_deque.push_front(a[i]);
        }
    }
    let res_vec: Vec<_> = res_deque.iter().collect();
    let res: String = res_vec.iter().join("");
    println!("{}", res);
}
