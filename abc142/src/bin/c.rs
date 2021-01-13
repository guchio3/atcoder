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
    let mut index_vec = vec![];
    for (i, &a_i) in a.iter().enumerate() {
        index_vec.push((i + 1, a_i));
    }
    index_vec.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
    let mut res_vec = vec![];
    for index_vec_i in index_vec {
        res_vec.push(index_vec_i.0);
    }
    println!("{}", res_vec.iter().join(" "));
}
