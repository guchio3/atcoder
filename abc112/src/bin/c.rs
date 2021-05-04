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
        mut xyh: [(i64, i64, i64); n]
    }
    xyh.sort_by(|x, y| y.2.partial_cmp(&x.2).unwrap());
    for x in 0..=100 {
        for y in 0..=100 {
            let mut hs = HashSet::new();
            for &(x_i, y_i, h_i) in xyh.iter() {
                let h_max = hs.iter().fold(0, |x, h_max| max(x, *h_max));
                let h;
                if h_i == 0 {
                    h = min(h_i + (x - x_i).abs() + (y - y_i).abs(), h_max);
                } else {
                    h = h_i + (x - x_i).abs() + (y - y_i).abs();
                }
                hs.insert(h);
            }

            if hs.len() == 1 {
                let hs: Vec<i64> = hs.into_iter().collect();
                println!("{} {} {}", x, y, hs[0]);
                return;
            }
        }
    }
}
