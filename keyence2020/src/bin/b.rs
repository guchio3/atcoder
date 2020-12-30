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
        xl: [(usize, usize); n],
    }
    let mut se = vec![];
    for &xl_i in xl.iter() {
        let x = xl_i.0;
        let l = xl_i.1;
        let s;
        if x < l {
            s = 0;
        } else {
            s = x - l;
        }
        let e = x + l;

        se.push((s, e));
    }
    se.sort_by(|se1, se2| se2.1.partial_cmp(&se1.1).unwrap());
}
