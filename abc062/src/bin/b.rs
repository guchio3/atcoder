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
        h: usize, w: usize,
        a: [Chars; h]
    }
    for i in 0..h + 2 {
        let mut tmp_res = vec![];
        for j in 0..w + 2 {
            let c;
            if i == 0 || i == h + 1 || j == 0 || j == w + 1 {
                c = '#';
            } else {
                c = a[i - 1][j - 1];
            }
            tmp_res.push(c);
        }
        println!("{}", tmp_res.into_iter().join(""));
    }
}
