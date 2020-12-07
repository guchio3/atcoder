#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        c: [[u8; 3]; 3]
    }

    let sum = c[0][0] + c[1][1] + c[2][2];
    if sum == c[0][1] + c[1][2] + c[2][0]
        && sum == c[1][0] + c[2][1] + c[0][2]
        && sum == c[0][2] + c[1][1] + c[2][0]
        && sum == c[0][1] + c[1][0] + c[2][2]
        && sum == c[0][0] + c[1][2] + c[2][1]
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
