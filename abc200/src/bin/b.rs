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

fn main() {
    input! {
        mut n: usize, k: usize
    }
    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n *= 1000;
            n += 200;
        }
    }
    println!("{}", n);
}
