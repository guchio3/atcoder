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
        w: usize, a: usize, b: usize
    }
    let left = min(a, b);
    let right = max(a, b);
    if left + w < right {
        println!("{}", right - left - w);
    } else {
        println!("0");
    }
}
