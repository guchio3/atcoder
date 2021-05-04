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
        mut b: i64, mut c: i64
    }
    let mut res = 1;
    if b == 0 {
        res += (c - 1) / 2 + c / 2;
    } else if b < 0 {
        res += (c > 0) as i64;
        res += (c - 1) / 2 + c / 2;
        res += min(b.abs() - 1, max(0, c - 2) / 2) + min(b.abs() - 1, max(0, c - 1) / 2);
        res += (b <= c / 2) as i64;
    } else {
        res += (c > 0) as i64;
        res += max(0, c - 2) / 2 + max(0, c - 1) / 2;
        res += min(b - 1, (c - 1) / 2) + min(b - 1, c / 2);
        res += (b <= c / 2) as i64;
    }
    println!("{}", res);
}
