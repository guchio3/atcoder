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
        n: usize, m: usize,
        mut ab: [(usize, usize); n]
    }
    ab.sort();
    let mut j = 0;
    let mut res = 0;
    for _i in 0..m {
        if ab[j].1 == 0 {
            j += 1;
        }
        res += ab[j].0;
        ab[j].1 -= 1;
    }
    println!("{}", res);
}
