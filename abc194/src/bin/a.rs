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
        a: usize, b: usize
    }
    if a + b >= 15 && b >= 8 {
        println!("1");
    } else if a + b >= 10 && b >= 3 {
        println!("2");
    } else if a + b >= 3 {
        println!("3");
    } else {
        println!("4");
    }
}
