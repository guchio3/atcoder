#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        a: usize, b: usize, q: usize,
        s: [i64; a],
        t: [i64; b],
        x: [i64; q]
    }
    let mut indices = vec![];
    for i in 0..q {
        indices.push((x[i], i));
    }
    indices.sort();

    let mut stx = vec![];
    for &s_i in s.iter() {
        stx.push((s_i, 's'));
    }
    for &t_i in t.iter() {
        stx.push((t_i, 't'));
    }
    for &x_i in x.iter() {
        // x is the last char cmp x & t
        stx.push((x_i, 'x'));
    }
    stx.sort();

    let mut left_nearest_s = vec![];
    let mut left_nearest_t = vec![];
    let mut latest_s_i = -1_000_000_000_000_000;
    let mut latest_t_i = -1_000_000_000_000_000;
    for &(place, kind) in stx.iter() {
        if kind == 'x' {
            left_nearest_s.push(place - latest_s_i);
            left_nearest_t.push(place - latest_t_i);
        } else if kind == 's' {
            latest_s_i = place;
        } else {
            latest_t_i = place;
        }
    }

    let mut stx = vec![];
    for s_i in s {
        stx.push((s_i, 3));
    }
    for t_i in t {
        stx.push((t_i, 2));
    }
    for x_i in x {
        // x is the last char cmp x & t
        stx.push((x_i, 1));
    }
    stx.sort_by(|x, y| y.partial_cmp(&x).unwrap());
    let mut right_nearest_s = vec![];
    let mut right_nearest_t = vec![];
    let mut latest_s_i = 1_000_000_000_000_000;
    let mut latest_t_i = 1_000_000_000_000_000;
    for (place, kind) in stx {
        if kind == 1 {
            right_nearest_s.push(latest_s_i - place);
            right_nearest_t.push(latest_t_i - place);
        } else if kind == 3 {
            latest_s_i = place;
        } else {
            latest_t_i = place;
        }
    }
    right_nearest_s = right_nearest_s.into_iter().rev().collect();
    right_nearest_t = right_nearest_t.into_iter().rev().collect();

    let mut res = vec![1_000_000_000_000_000; q];
    for i in 0..q {
        let index = indices[i].1;
        let left_nearest_s_i = left_nearest_s[i];
        let left_nearest_t_i = left_nearest_t[i];
        let right_nearest_s_i = right_nearest_s[i];
        let right_nearest_t_i = right_nearest_t[i];
        res[index] = min(res[index], max(left_nearest_s_i, left_nearest_t_i));
        res[index] = min(res[index], max(right_nearest_s_i, right_nearest_t_i));
        res[index] = min(res[index], left_nearest_s_i * 2 + right_nearest_t_i);
        res[index] = min(res[index], left_nearest_s_i + right_nearest_t_i * 2);
        res[index] = min(res[index], left_nearest_t_i * 2 + right_nearest_s_i);
        res[index] = min(res[index], left_nearest_t_i + right_nearest_s_i * 2);
    }

    for res_i in res {
        println!("{}", res_i);
    }
}
