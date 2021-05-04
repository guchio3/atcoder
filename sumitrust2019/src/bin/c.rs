#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        x: usize
    }
    let mut exists = vec![false; 100_001];
    let nums = vec![100, 101, 102, 103, 104, 105];
    exists[0] = true;
    for i in 0..=100_000 {
        for &num in nums.iter() {
            if i >= num && exists[i - num] {
                exists[i] = true;
                break;
            }
        }
    }
    if exists[x] {
        println!("1");
    } else {
        println!("0");
    }
}
