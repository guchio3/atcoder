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
        n: usize, k: usize,
        v: [i64; n]
    }
    let mut res = 0;
    // i 回操作
    for i in 1..=k {
        // j 回 pop
        for j in 0..=min(i, n) {
            // 左側が k 回
            for k in 0..=j {
                let mut used_nums = vec![];
                let mut tmp_res = 0;
                for l in 0..k {
                    used_nums.push(v[l]);
                    tmp_res += v[l];
                }
                for l in n - (j - k)..n {
                    used_nums.push(v[l]);
                    tmp_res += v[l];
                }
                used_nums.sort_by(|x, y| y.partial_cmp(&x).unwrap());
                for _ in 0..i - j {
                    if used_nums.len() > 0 && used_nums[used_nums.len() - 1] < 0 {
                        tmp_res -= used_nums.pop().unwrap();
                    }
                }
                res = max(res, tmp_res);
            }
        }
    }
    println!("{}", res);
}
