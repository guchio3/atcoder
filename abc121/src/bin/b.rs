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
        n: usize, m: usize, c: i64,
        b: [i64; m],
        a: [[i64; m]; n]
    }
    let mut res = 0;
    for i in 0..n {
        let mut num = c;
        for j in 0..m {
            num += a[i][j] * b[j];
        }
        if num > 0 {
            res += 1;
        }
    }
    println!("{}", res);
}
