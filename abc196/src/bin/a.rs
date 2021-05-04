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
        a: i64, b: i64,
        c: i64, d: i64
    }
    let mut res = -100000000;
    for i in a..=b {
        for j in c..=d {
            res = max(res, i - j);
        }
    }
    println!("{}", res);
}
