#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize, k: u64,
        a: [u64; n]
    }
    let mut non_res: usize = 0;
    let mut total: usize = 0;
    let mut temp_sum = 0;
    let mut left = 0;

    for right in 0..a.len() {
        temp_sum += a[right];
        while left <= right && temp_sum >= k {
            temp_sum -= a[left];
            left += 1;
        }
        non_res += right + 1 - left;
        total += right + 1;
    }

    println!("{}", total - non_res);
}
