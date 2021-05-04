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
        mut n: Chars
    }
    let mut num_chars = vec![];
    n = n.into_iter().rev().collect();
    let mut i = 0;
    while i < n.len() && n[i] == '0' {
        i += 1;
    }
    while i < n.len() {
        num_chars.push(n[i]);
        i += 1;
    }
    for j in 0..num_chars.len() / 2 {
        if num_chars[j] != num_chars[num_chars.len() - j - 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
