#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h]
    }
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                let mut double_black = false;
                if i > 0 && s[i - 1][j] == '#' {
                    double_black = true;
                }
                if j > 0 && s[i][j - 1] == '#' {
                    double_black = true;
                }
                if i < h - 1 && s[i + 1][j] == '#' {
                    double_black = true;
                }
                if j < w - 1 && s[i][j + 1] == '#' {
                    double_black = true;
                }
                if !double_black {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
