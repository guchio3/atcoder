#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize, k: usize,
        x: [i64; n]
    }
    // window 切ってスライドさせていくイメージ
    // 左から詰めるか右から詰めるか
    let mut res = (x[n - 1] - x[0]) * 2;
    for start_i in 0..=(n - k) {
        let end_i = start_i + k - 1;
        let tmp_res;
        if x[end_i] <= 0 {
            tmp_res = -1 * x[start_i];
        } else if x[start_i] >= 0 {
            tmp_res = x[end_i];
        } else {
            tmp_res = min(-2 * x[start_i] + x[end_i], -1 * x[start_i] + 2 * x[end_i]);
        }
        res = min(res, tmp_res);
    }
    println!("{}", res);
}
