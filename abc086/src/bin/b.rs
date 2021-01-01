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
        mut a: String, b: String
    }
    a.push_str(&b);
    let num = a.parse::<f64>().unwrap();
    let root = num.powf(0.5);
    if (num / root).ceil() as usize == root as usize {
        println!("Yes");
    } else {
        println!("No");
    }
}
