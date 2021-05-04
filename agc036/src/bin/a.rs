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
        s: usize
    }
    if s == 1_000_000_000_000_000_000 {
        println!(
            "{} {} {} {} {} {}",
            0, 0, 1_000_000_000, 0, 0, 1_000_000_000
        );
        return;
    }
    let x_3 = (1_000_000_000 - s % 1_000_000_000) % 1_000_000_000;
    let y_3 = (s + x_3) / 1_000_000_000;
    println!("0 0 1000000000 1 {} {}", x_3, y_3);
}
