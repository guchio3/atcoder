#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        mut d: [usize; n]
    }
    if d[0] != 0 {
        println!("0");
        return;
    }
    d.sort();

    let mut res = 1;
    let mut border = 1;
    let mut border_cnt = 0;
    let mut bef_border_cnt = 1;
    for i in 1..d.len() {
        let d_i = d[i];
        if d_i == border {
            border_cnt += 1;
        } else if d_i == border + 1 {
            for _ in 0..border_cnt {
                res = res * bef_border_cnt % MOD;
            }
            border += 1;
            bef_border_cnt = border_cnt;
            border_cnt = 1;
        } else {
            println!("0");
            return;
        }
    }
    for _ in 0..border_cnt {
        res = res * bef_border_cnt % MOD;
    }
    println!("{}", res);
}
