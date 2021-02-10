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
        x: f64, y: f64, r: f64
    }
    let mut x = (x * 10000.).round() as i64;
    let mut y = (y * 10000.).round() as i64;
    let r = (r * 10000.).round() as i64;
    let max_abs = (max(x.abs(), y.abs()) / 10000 + 1) * 10000;
    x += (r / 10000 + 1) * 10000 + max_abs;
    y += (r / 10000 + 1) * 10000 + max_abs;
    let mut left_pow_points = vec![];
    let mut left_side = x % 10000;
    while left_side <= r {
        left_pow_points.push(left_side.pow(2));
        left_side += 10000;
    }
    let mut right_pow_points = vec![];
    let mut right_side = 10000 * (x % 10000 != 0) as i64 - x % 10000;
    while right_side <= r {
        right_pow_points.push(right_side.pow(2));
        right_side += 10000;
    }
    let mut res: i64 = 0;
    let diff = (y + r) / 10000 - (y - r) / 10000 - ((y - r) % 10000 != 0) as i64;
    for i in 0..=diff {
        let yy = ((y - r) / 10000 + ((y - r) % 10000 != 0) as i64) * 10000 + (i as i64) * 10000;
        let xx = r.pow(2) - (y - yy).pow(2);
        let left: i64;
        let right: i64;
        match left_pow_points.binary_search(&xx) {
            Ok(v) => {
                left = v as i64 + 1;
            }
            Err(v) => {
                left = v as i64;
            }
        }
        match right_pow_points.binary_search(&xx) {
            Ok(v) => {
                right = v as i64 + 1;
            }
            Err(v) => {
                right = v as i64;
            }
        }
        res += right + left;
        if x % 10000 == 0 {
            res -= 1;
        }
    }
    println!("{}", res);
}
