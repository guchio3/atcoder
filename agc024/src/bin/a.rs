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
        a: i64, b: i64, _c: i64, k: i64
    }
    // a / b / c
    // b+c / a+c / a+b
    // 2a+b+c / a+2b+c / a+b+2c
    // 2a+3b+3c / 3a+2b+3c / 3a+3b+2c
    // 6a+5b+5c / 5a+6b+5c / 5a+5b+6c
    let res;
    if k % 2 == 0 {
        res = a - b;
    } else {
        res = b - a;
    }
    println!("{}", res);
}
