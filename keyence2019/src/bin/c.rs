#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n]
    }
    let a_sum: i64 = a.iter().sum();
    let b_sum: i64 = b.iter().sum();
    if a_sum < b_sum {
        println!("-1");
    } else {
        let mut costs = vec![];
        for i in 0..n {
            costs.push(a[i] - b[i]);
        }
        costs.sort();

        let mut res = 0;
        let mut total_cost = 0;
        for &cost in costs.iter() {
            if cost < 0 {
                total_cost -= cost;
                res += 1;
            }
        }

        for cost in costs.into_iter().rev() {
            if cost > 0 {
                if total_cost > 0 {
                    total_cost -= cost;
                    res += 1;
                }
            }
        }
        println!("{}", res);
    }
}
