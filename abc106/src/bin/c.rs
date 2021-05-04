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
        s: Chars,
        k: usize
    }
    for i in 0..k {
        if s[i] != '1' {
            println!("{}", s[i]);
            return;
        }
    }
    println!("{}", s[k - 1]);
}
