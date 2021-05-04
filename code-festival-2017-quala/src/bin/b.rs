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
        n: usize, m: usize, k: usize
    }
    for i in 0..=n {
        let black_num = i * m;
        for j in 0..=m {
            if black_num + (n - i) * j - i * j == k {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
