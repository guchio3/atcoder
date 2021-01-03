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
        n: usize,
        mut d: [usize; n]
    }
    d.sort();
    let index = n / 2;
    let left = d[index - 1];
    let right = d[index];
    println!("{}", right - left);
}
