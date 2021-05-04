#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize
    }
    let mut digit_nums = vec![];
    let mut n_copy = n;
    while n_copy > 0 {
        digit_nums.push(n_copy % 10);
        n_copy /= 10;
    }
    let mut res = 0;
    for i in 0..digit_nums.len() {
        res += digit_nums[i];
    }

    println!(
        "{}",
        max(
            res,
            9 * (digit_nums.len() - 1) + digit_nums[digit_nums.len() - 1] - 1
        )
    )
}
