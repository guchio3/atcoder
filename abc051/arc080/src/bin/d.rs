#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        h: usize, w: usize,
        n: usize,
        a: [usize; n]
    }
    let mut res = vec![vec![0; w]; h];

    let mut direction = 0;

    let mut i = 0;
    let mut j = 0;
    for (color, &a_i) in a.iter().enumerate() {
        for _ in 0..a_i {
            res[i][j] = color + 1;
            match direction {
                0 => {
                    if j == w - 1 || res[i][j + 1] != 0 {
                        i += 1;
                        direction = 1;
                    } else {
                        j += 1;
                    }
                }
                1 => {
                    if i == h - 1 || res[i + 1][j] != 0 {
                        j -= 1;
                        direction = 2;
                    } else {
                        i += 1;
                    }
                }
                2 => {
                    if j == 0 || res[i][j - 1] != 0 {
                        i -= 1;
                        direction = 3;
                    } else {
                        j -= 1;
                    }
                }
                3 => {
                    if i == 0 || res[i - 1][j] != 0 {
                        j += 1;
                        direction = 0;
                    } else {
                        i -= 1;
                    }
                }
                _ => {
                    panic!("should not occur!");
                }
            }
        }
    }

    for res_i in res {
        let res_str: String = res_i.iter().join(" ");
        println!("{}", res_str);
    }
}
