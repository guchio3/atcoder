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
        t: [usize; n]
    }
    if n == 1 {
        println!("{}", t[0]);
    } else {
        let mut res = lcm(t[0], t[1]);
        for i in 2..n {
            res = lcm(res, t[i]);
        }
        println!("{}", res);
    }
}
