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
        a: usize, b: usize, c: usize
    }
    if c < a + b {
        println!("No");
    } else {
        if 4 * a * b < (c - a - b).pow(2) {
            println! {"Yes"};
        } else {
            println!("No");
        }
    }
}
