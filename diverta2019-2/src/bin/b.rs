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
        xy: [(i64, i64); n]
    }
    let mut res = n;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (xi, yi) = xy[i];
            let (xj, yj) = xy[j];
            let p = xj - xi;
            let q = yj - yi;
            let mut tmp_minus_num = 0;
            let mut ined = vec![false; n];
            let mut outed = vec![false; n];
            for k in 0..n {
                for l in 0..n {
                    if k == l || ined[k] || outed[l] {
                        continue;
                    }
                    if ((xy[l].0 - xy[k].0) == p) && ((xy[l].1 - xy[k].1) == q) {
                        ined[k] = true;
                        outed[l] = true;
                        tmp_minus_num += 1;
                    }
                }
            }
            res = min(res, n - tmp_minus_num);
        }
    }
    println!("{}", res);
}
