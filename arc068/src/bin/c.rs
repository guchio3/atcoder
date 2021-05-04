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
        x: usize
    }
    let mut res = 2 * (x / 11);
    let rest = x % 11;
    if rest > 0 {
        if rest > 6 {
            res += 2;
        } else {
            res += 1;
        }
    }
    println!("{}", res);
}
