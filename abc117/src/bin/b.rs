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
        n: usize,
        mut l: [usize; n]
    }
    l.sort();
    let mut all_sum = 0;
    for i in 0..n - 1 {
        all_sum += l[i];
    }
    if l[n - 1] < all_sum {
        println!("Yes");
    } else {
        println!("No");
    }
}
