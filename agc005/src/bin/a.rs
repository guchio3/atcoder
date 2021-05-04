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
        x: Chars
    }

    let mut res = VecDeque::new();
    for x_i in x.into_iter() {
        if res.len() == 0 {
            res.push_back(x_i);
        } else {
            if x_i == 'T' {
                if res.back() == Some(&'S') {
                    res.pop_back();
                } else {
                    res.push_back(x_i);
                }
            } else {
                res.push_back(x_i);
            }
        }
    }
    println!("{}", res.len());
}
