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
        mut v: [usize; n]
    }
    v.sort();
    let mut res = v[0] as f64;
    for i in 1..v.len() {
        let v_i = v[i];
        res = (res + v_i as f64) / 2.;
    }
    println!("{}", res);
}
