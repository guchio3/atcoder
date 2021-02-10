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
        a: i64, b: i64
    }
    let prod = a * b;
    if prod <= 0 {
        println!("Zero");
    } else if a > 0 {
        println!("Positive");
    } else {
        if (b - a) % 2 == 0 {
            println!("Negative");
        } else {
            println!("Positive");
        }
    }
}
