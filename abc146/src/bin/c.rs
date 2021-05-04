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
        a: usize, b: usize, x: usize
    }
    let mut left = 0;
    let mut right = 1_000_000_000;
    while right - left > 1 {
        let n = (left + right) / 2;
        let mut digit = 0;
        let mut digit_num = n;
        while digit_num > 0 {
            digit += 1;
            digit_num /= 10;
        }
        let price = a * n + b * digit;
        if price >= x {
            right = n;
        } else {
            left = n;
        }
    }
    let mut digit = 0;
    let mut digit_num = right;
    while digit_num > 0 {
        digit += 1;
        digit_num /= 10;
    }
    let price = a * right + b * digit;
    if price > x {
        println!("{}", left);
    } else {
        println!("{}", right);
    }
}
