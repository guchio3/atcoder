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
        x: Chars
    }
    let mut reses = vec![];
    for i in 0..x.len() {
        if x[i] != '.' {
            reses.push(x[i]);
        } else {
            break;
        }
    }
    println!("{}", reses.into_iter().join(""));
}
