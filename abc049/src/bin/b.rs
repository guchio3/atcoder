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
        h: usize, w: usize,
        c: [Chars; h]
    }
    for i in 0..h {
        println!("{}", c[i].iter().map(|x| *x).join(""));
        println!("{}", c[i].iter().map(|x| *x).join(""));
    }
}
