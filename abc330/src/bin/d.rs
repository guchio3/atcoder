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
        n: usize,
        s: [Chars; n]
    }
    let mut row_cnts = vec![0; n];
    let mut col_cnts = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                row_cnts[i] += 1;
                col_cnts[j] += 1;
            }
        }
    }

    let mut res: i64 = 0;

    for i in 0..n {
        if row_cnts[i] < 2 {
            continue;
        }
        for j in 0..n {
            if s[i][j] == 'o' {
                res += (row_cnts[i] - 1) * (col_cnts[j] - 1);
            }
        }
    }
    println!("{}", res);
}
