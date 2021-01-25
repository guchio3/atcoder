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
        n: usize, k: usize, s: f64
    }
    let mut res = vec![];
    if k == n {
        for _i in 0..n {
            res.push(s as usize);
        }
    } else if k == 0 {
        for _i in 0..n {
            if s == 1_000_000_000. {
                res.push(1);
            } else {
                res.push(1_000_000_000);
            }
        }
    } else {
        for i in 0..n {
            if i <= k {
                if s == 1. {
                    if i == k {
                        res.push(1_000_000_000);
                    } else {
                        res.push(1);
                    }
                } else {
                    if i % 2 == 0 {
                        res.push(((s / 2.).floor()) as usize);
                    } else {
                        res.push(((s / 2.).ceil()) as usize);
                    }
                }
            } else {
                if s == 1_000_000_000. {
                    res.push(1_000_000_000 - 1);
                } else {
                    res.push(1_000_000_000);
                }
            }
        }
    }
    println!("{}", res.into_iter().join(" "));
}
