#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        sx: i64, sy: i64, tx: i64, ty: i64
    }
    let mut res = vec![];
    for _ in 0..(tx - sx) {
        res.push('R');
    }
    for _ in 0..(ty - sy) {
        res.push('U');
    }
    for _ in 0..(tx - sx) {
        res.push('L');
    }
    for _ in 0..(ty - sy) {
        res.push('D');
    }
    res.push('D');
    for _ in 0..((tx - sx) + 1) {
        res.push('R');
    }
    for _ in 0..((ty - sy) + 1) {
        res.push('U');
    }
    res.push('L');
    res.push('U');
    for _ in 0..((tx - sx) + 1) {
        res.push('L');
    }
    for _ in 0..((ty - sy) + 1) {
        res.push('D');
    }
    res.push('R');
    let res_str: String = res.iter().collect();
    println!("{}", res_str);
}
