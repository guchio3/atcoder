#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        s: Chars,
    }
    let mut tmp_res = vec![];
    for s_i in s {
        if s_i == 'B' {
            if tmp_res.len() != 0 {
                tmp_res.pop();
            }
        } else {
            tmp_res.push(s_i);
        }
    }
    let res: String = tmp_res.iter().collect();
    println!("{}", res);
}
