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
        a: usize, b: usize
    }
    let mut first = 0;
    let mut second = 0;
    for i in 0..a {
        first += b * 10.pow(i as u32);
    }
    for i in 0..b {
        second += a * 10.pow(i as u32);
    }
    println!("{}", min(first.to_string(), second.to_string()));
}
