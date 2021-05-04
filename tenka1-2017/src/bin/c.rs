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
        n: usize
    }
    for i in 1..=3500 {
        for j in 1..=3500 {
            if 4 * i * j < n * (i + j) {
                continue;
            }
            let num1 = n * i * j;
            let num2 = 4 * i * j - n * (i + j);
            if num2 > 0 && num1 % num2 == 0 {
                println!("{} {} {}", num1 / num2, i, j);
                return;
            }
        }
    }
}
