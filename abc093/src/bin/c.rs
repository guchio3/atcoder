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
        a: usize, b: usize, c: usize
    }
    let mut res = 0;
    let mut vec = vec![];
    vec.push(a);
    vec.push(b);
    vec.push(c);
    vec.sort();
    while vec[1] < vec[2] {
        vec[0] += 1;
        vec[1] += 1;
        res += 1;
    }
    while vec[0] < vec[1] {
        vec[0] += 2;
        res += 1;
    }
    vec.sort();
    while vec[1] < vec[2] {
        vec[0] += 1;
        vec[1] += 1;
        res += 1;
    }
    println!("{}", res);
}
