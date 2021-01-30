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
        n: usize, s: i64, d: i64,
        xy: [(i64, i64); n]
    }
    for (x_i, y_i) in xy {
        if x_i >= s || y_i <= d {
            continue;
        }
        println!("Yes");
        return;
    }
    println!("No");
}
