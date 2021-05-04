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
        a: i64, b: i64, k: i64
    }
    let mut set = HashSet::new();
    for i in 0..k {
        set.insert(min(a + i, b));
        set.insert(max(b - i, a));
    }
    let mut reses: Vec<i64> = set.into_iter().collect();
    reses.sort();
    for res in reses {
        println!("{}", res);
    }
}
