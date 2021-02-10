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
        p: [usize; n]
    }
    let mut res = 0;
    let mut p_min = 99999999;
    for p_i in p {
        p_min = min(p_min, p_i);
        if p_min >= p_i {
            res += 1;
        }
    }
    println!("{}", res);
}
