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
        n: usize, mut k: usize,
        mut h: [usize; n]
    }
    h.sort();
    let mut res = 0;
    for h_i in h.into_iter().rev() {
        if k > 0 {
            k -= 1;
        } else {
            res += h_i;
        }
    }
    println!("{}", res);
}
