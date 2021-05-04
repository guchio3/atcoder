#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use ordered_float::NotNan;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn is_possible_sep(a: &Vec<usize>, k: usize, x: usize) -> bool {
    let mut i = 0;
    let mut bef_place = 0;
    let mut cum_cm = 0;
    for _ in 0..k + 1 {
        while i < a.len() && cum_cm < x {
            cum_cm += a[i] - bef_place;
            bef_place = a[i];
            i += 1;
        }
        if i >= a.len() && cum_cm < x {
            return false;
        }
        cum_cm = 0;
    }
    true
}

fn main() {
    input! {
        n: usize, l: usize,
        k: usize,
        mut a: [usize; n]
    }
    a.push(l);
    let mut ll = 0;
    let mut rr = 1_000_000_001;
    while rr - ll > 1 {
        let x = (ll + rr) / 2;
        if is_possible_sep(&a, k, x) {
            ll = x;
        } else {
            rr = x;
        }
    }
    println!("{}", ll);
}
