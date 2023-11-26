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
        d: i64,
    }
    let mut ys = vec![];
    for y in 0..=1_500_000 {
        ys.push(y.pow(2));
    }
    let mut res = d;
    for x in 0..=1_500_000 {
        let base = x.pow(2) - d;
        if base > 0 {
            res = min(res, base);
        } else {
            let idx = ys.binary_search(&-base);
            match idx {
                Ok(_) => res = 0,
                Err(actual_idx) => {
                    if actual_idx == 0 {
                        res = min(res, (base + ys[actual_idx]).abs());
                    } else {
                        res = min(
                            min(res, (base + ys[actual_idx]).abs()),
                            (base + ys[actual_idx - 1]).abs(),
                        );
                    }
                    // println!("{}: {} => {}", x, actual_idx, res);
                }
            }
        }
    }
    println!("{}", res);
}
