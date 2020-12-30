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
        h: [usize; n]
    }
    let mut res = 0;
    let max_h = *h.iter().max().unwrap();
    let mut heights = vec![0; n];
    for _ in 0..=max_h {
        let mut cum = 0;
        for i in 0..n {
            if heights[i] < h[i] {
                heights[i] += 1;
                cum += 1;
            } else {
                if cum > 0 {
                    res += 1;
                    cum = 0;
                }
            }
        }
        if cum > 0 {
            res += 1;
        }
    }
    println!("{}", res);
}
