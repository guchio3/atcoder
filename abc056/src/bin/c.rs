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
        x: usize,
    }
    if x == 1 {
        println!("1");
        return;
    }
    let mut sum = 0;
    for i in 1..x {
        sum += i;
        if sum + i + 1 >= x {
            println!("{}", i + 1);
            return;
        }
    }
}
