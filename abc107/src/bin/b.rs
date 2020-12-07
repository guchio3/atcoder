#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h: usize, w: usize,
        mut a: [Chars; h]
    }
    for i in 0..h {
        let mut black_exists = false;
        for j in 0..w {
            if a[i][j] == '#' {
                black_exists = true;
                break;
            }
        }
        if !black_exists {
            for j in 0..w {
                a[i][j] = 'e';
            }
        }
    }

    for i in 0..w {
        let mut black_exists = false;
        for j in 0..h {
            if a[j][i] == '#' {
                black_exists = true;
                break;
            }
        }
        if !black_exists {
            for j in 0..h {
                a[j][i] = 'e';
            }
        }
    }

    for i in 0..h {
        let mut res_vec = vec![];
        for j in 0..w {
            if a[i][j] != 'e' {
                res_vec.push(a[i][j]);
            }
        }
        if res_vec.len() > 0 {
            let res: String = res_vec.iter().collect();
            println!("{}", res);
        }
    }
}
