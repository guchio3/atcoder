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
        n: usize, m: usize,
        ab: [(usize, usize); m],
        k: usize,
        cd: [(usize, usize); k]
    }
    let mut res = 0;
    for i in 0..(1 << k) {
        let mut tmp_res = 0;
        let mut has_ball = vec![false; n];
        for j in 0..k {
            if i & (1 << j) > 0 {
                has_ball[cd[j].1 - 1] = true;
            } else {
                has_ball[cd[j].0 - 1] = true;
            }
        }
        for &(a_i, b_i) in ab.iter() {
            if has_ball[a_i - 1] && has_ball[b_i - 1] {
                tmp_res += 1;
            }
        }
        res = max(res, tmp_res);
    }
    println!("{}", res);
}
