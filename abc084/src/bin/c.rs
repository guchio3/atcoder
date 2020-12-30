#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        csf: [(usize, usize, usize); n - 1]
    }

    let mut res = vec![];
    for i in 0..csf.len() {
        let mut t = 0;
        for j in i..csf.len() {
            let c_j = csf[j].0;
            let s_j = csf[j].1;
            let f_j = csf[j].2;
            if t < s_j {
                t += s_j - t;
            }
            if t % f_j != 0 {
                t += f_j - (t % f_j);
            }
            t += c_j;
        }
        res.push(t);
    }

    for res_i in res {
        println!("{}", res_i);
    }
    println!("0");
}
