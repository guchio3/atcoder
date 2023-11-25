#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        b: i64
    }

    for i in 1..=15 {
        let mut tmp = 1;
        for j in 0..i {
            tmp *= i;
        }
        if tmp == b {
            println!("{}", i);
            return;
        }
    }
    println!("{}", -1);
}
