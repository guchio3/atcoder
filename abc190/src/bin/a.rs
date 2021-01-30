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
        mut a: usize, mut b: usize, c: usize,
    }
    if c == 1 {
        a += 1;
    }
    loop {
        if a == 0 {
            println!("Aoki");
            return;
        }
        a -= 1;
        if b == 0 {
            println!("Takahashi");
            return;
        }
        b -= 1;
    }
}
