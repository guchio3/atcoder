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
        f: [[i64; 10]; n],
        p: [[i64; 11]; n]
    }
    let mut res: i64 = -1_000_000_000;
    for i in 1..(1 << 10) {
        let mut c = vec![0; n];
        for j in 0..10 {
            for k in 0..n {
                if f[k][j] == 1 && i & (1 << j) > 0 {
                    c[k] += 1;
                }
            }
        }
        let mut tmp_res = 0;
        for k in 0..n {
            tmp_res += p[k][c[k]];
        }
        res = max(res, tmp_res);
    }
    println!("{}", res);
}
