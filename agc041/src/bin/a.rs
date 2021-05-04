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
        n: usize, a: usize, b: usize
    }
    let larger = max(a, b);
    let smaller = min(a, b);
    let diff = larger - smaller;
    if diff % 2 == 0 {
        println!("{}", diff / 2);
    } else {
        if n - larger <= smaller - 1 {
            println!("{}", n - larger + (diff - 1) / 2 + 1);
        } else {
            println!("{}", smaller - 1 + (diff - 1) / 2 + 1);
        }
    }
}
