#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn get_first_divisors(num: usize) -> usize {
    let mut i = ((num as f64).sqrt().ceil()) as usize;
    let res;
    loop {
        if num % i == 0 {
            res = i;
            break;
        }
        i -= 1;
    }
    res
}

fn main() {
    input! {
        s: usize
    }
    let x_3 = s / 1_000_000_000;
    let y_3 = s % 1_000_000_000;
    println!("0 0 1000000000 1 {} {}", x_3, y_3);
}
