#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        mut n: usize,
        a: [usize; n]
    }
    let mut has_two = 0;
    let mut has_four = 0;
    for a_i in a {
        if a_i % 4 == 0 {
            has_four += 1;
        } else if a_i % 2 == 0 {
            has_two += 1;
        }
    }
    if has_two == 0 {
        n -= has_two;
        if has_four + 1 >= n - has_four {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        n -= has_two;
        if has_four >= n - has_four {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
