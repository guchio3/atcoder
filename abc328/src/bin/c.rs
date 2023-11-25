#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::vec;

fn main() {
    input! {
        n: usize, q: usize,
        s: Chars,
        lr: [(usize, usize); q]
    }

    let mut cum_pairs: Vec<usize> = vec![0; n];
    for i in 1..n {
        if s[i - 1] == s[i] {
            cum_pairs[i] = cum_pairs[i - 1] + 1;
        } else {
            cum_pairs[i] = cum_pairs[i - 1];
        }
    }

    for i in 0..q {
        let (l, r) = (lr[i].0, lr[i].1);
        if l == 0 {
            println!("{}", cum_pairs[r - 1]);
        } else {
            println!("{}", cum_pairs[r - 1] - cum_pairs[l - 1]);
        }
    }
}
