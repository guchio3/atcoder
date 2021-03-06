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
        n: usize, _m: usize,
    }
    let mut liked = HashMap::new();
    for _i in 0..n {
        input! {
            k: usize,
            a: [usize; k]
        }
        for a_i in a {
            *liked.entry(a_i).or_insert(0) += 1;
        }
    }
    let mut res = 0;
    for (_key, value) in liked {
        if value == n {
            res += 1;
        }
    }
    println!("{}", res);
}
