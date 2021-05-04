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
        a: usize, b: usize, c: usize
    }
    if a % 2 == 0 || b % 2 == 0 || c % 2 == 0 {
        println!("0");
    } else {
        println!("{}", min(c * a, min(a * b, b * c)));
    }
}
