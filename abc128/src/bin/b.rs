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
        sp: [(String, i64); n]
    }
    let mut sp2 = vec![];
    for i in 0..n {
        sp2.push((sp[i].0.clone(), -sp[i].1, i + 1))
    }
    sp2.sort();
    for i in 0..n {
        println!("{}", sp2[i].2);
    }
}
