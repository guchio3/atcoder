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
        n: usize
    }
    let mut res = n;
    for i in 1..n {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            res = min(res, (i - 1) + (n / i - 1));
        }
    }
    println!("{}", res);
}
