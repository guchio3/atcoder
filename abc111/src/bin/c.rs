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
        v: [usize; n]
    }
    let mut even_cnts = HashMap::new();
    let mut odd_cnts = HashMap::new();
    for i in 0..n / 2 {
        *even_cnts.entry(v[2 * i]).or_insert(0) += 1;
        *odd_cnts.entry(v[2 * i + 1]).or_insert(0) += 1;
    }
    let mut even_cnts_vec = vec![];
    for (key, value) in even_cnts {
        even_cnts_vec.push((value, key))
    }
    even_cnts_vec.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap());
    let mut odd_cnts_vec = vec![];
    for (key, value) in odd_cnts {
        odd_cnts_vec.push((value, key))
    }
    odd_cnts_vec.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap());
    let res;
    if even_cnts_vec.len() > 1 && odd_cnts_vec.len() > 1 {
        if even_cnts_vec[0].1 == odd_cnts_vec[0].1 {
            if even_cnts_vec[0].0 + odd_cnts_vec[1].0 > even_cnts_vec[1].0 + odd_cnts_vec[0].0 {
                res = n - even_cnts_vec[0].0 - odd_cnts_vec[1].0;
            } else {
                res = n - even_cnts_vec[1].0 - odd_cnts_vec[0].0;
            }
        } else {
            res = n - even_cnts_vec[0].0 - odd_cnts_vec[0].0;
        }
    } else if even_cnts_vec.len() > 1 {
        if even_cnts_vec[0].1 == odd_cnts_vec[0].1 {
            res = n - even_cnts_vec[1].0 - odd_cnts_vec[0].0;
        } else {
            res = n - even_cnts_vec[0].0 - odd_cnts_vec[0].0;
        }
    } else if even_cnts_vec.len() > 1 {
        if even_cnts_vec[0].1 == odd_cnts_vec[0].1 {
            res = n - even_cnts_vec[0].0 - odd_cnts_vec[1].0;
        } else {
            res = n - even_cnts_vec[0].0 - odd_cnts_vec[0].0;
        }
    } else {
        if even_cnts_vec[0].1 == odd_cnts_vec[0].1 {
            res = n - max(even_cnts_vec[0].0, odd_cnts_vec[0].0);
        } else {
            res = n - even_cnts_vec[0].0 - odd_cnts_vec[0].0;
        }
    }
    println!("{}", res);
}
