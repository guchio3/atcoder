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
        s: Chars,
        t: Chars
    }
    'outer: for i in 0..s.len() {
        for j in 0..s.len() {
            if s[(i + j) % s.len()] != t[j] {
                continue 'outer;
            }
        }
        println!("Yes");
        return;
    }
    println!("No");
}
