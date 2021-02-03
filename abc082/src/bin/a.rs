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
        a: [usize; n]
    }
    return;
    let mut cnt_dict = HashMap::new();
    for a_i in a {
        (*cnt_dict.entry(a_i).or_insert(0)) += 1;
    }
    let mut res = 0;
    for (key, value) in cnt_dict {
        if value >= key {
            res += value - key;
        } else {
            res += value;
        }
    }
    println!("{}", res);
}
