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
        w: [Chars; n]
    }
    let mut occurred = HashSet::new();
    let mut bef_end = w[0][0];
    for w_i in w {
        if occurred.contains(&w_i) || bef_end != w_i[0] {
            println!("No");
            return;
        }
        bef_end = w_i[w_i.len() - 1];
        occurred.insert(w_i);
    }
    println!("Yes");
}
