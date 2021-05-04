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
        a: usize, b: usize, c: usize, x: usize, y: usize
    }
    let mut res = 0;
    if a + b > c * 2 {
        res += c * min(x, y) * 2;
        if x > y {
            if a > 2 * c {
                res += (x - y) * 2 * c;
            } else {
                res += (x - y) * a;
            }
        } else {
            if b > 2 * c {
                res += (y - x) * 2 * c;
            } else {
                res += (y - x) * b;
            }
        }
    } else {
        res += a * x + b * y;
    }
    println!("{}", res);
}
