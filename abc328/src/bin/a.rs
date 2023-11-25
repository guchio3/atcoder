#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize, x: usize,
        s: [usize; n]
    }

    let mut res = 0;
    for i in 0..n {
        if s[i] <= x {
            res += s[i];
        }
    }

    println!("{}", res);
}
