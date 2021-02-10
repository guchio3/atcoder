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
        v: usize, t: usize, s: usize, d: usize,
    }
    if v * t <= d && d <= v * s {
        println!("No");
    } else {
        println!("Yes");
    }
}
