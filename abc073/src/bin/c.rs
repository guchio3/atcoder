#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut dict = HashMap::new();
    for a_i in a {
        *dict.entry(a_i).or_insert(0) += 1;
    }

    let mut res = 0;
    for (_key, value) in dict {
        if value % 2 != 0 {
            res += 1;
        }
    }
    println!("{}", res);
}
