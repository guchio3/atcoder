#![allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;

fn main() {
    input! {
        a: i32, b: i32, c: i32, d: i32, 
    }

    println!("{}", a * d - b * c);
}
