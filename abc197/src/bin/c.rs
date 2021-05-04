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
        a: [usize; n]
    }
    if n == 1 {
        println!("{}", a[0]);
    } else {
        let mut res: usize = 18446744073709551615;
        for i in 0..(1 << n - 1) {
            let mut tmp_res = 0;
            let mut tmp = a[0];
            for j in 0..n - 1 {
                if i & (1 << j) > 0 {
                    tmp_res ^= tmp;
                    tmp = a[j + 1];
                } else {
                    tmp |= a[j + 1];
                }
            }
            tmp_res ^= tmp;
            res = min(res, tmp_res);
        }
        println!("{}", res);
    }
}
