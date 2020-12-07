#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize, m: usize,
        a: [Chars; n],
        b: [Chars; m]
    }

    'first: for i in 0..n {
        if n < m + i {
            break;
        }
        'second: for j in 0..a[i].len() {
            for k in 0..m {
                if a[i + k].len() < b[k].len() + j {
                    continue 'first;
                }
                for l in 0..b[k].len() {
                    if a[i + k][j + l] != b[k][l] {
                        continue 'second;
                    }
                }
            }
            println!("Yes");
            return;
        }
    }

    println!("No");
}
