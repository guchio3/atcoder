#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        h: usize, w: usize,
        a: [Chars; h]
    }
    let mut sharp_cnt = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                sharp_cnt += 1;
            }
        }
    }
    if sharp_cnt != h + w - 1 || a[0][0] != '#' {
        println!("Impossible");
        return;
    } else {
        let mut curr_i = 0;
        let mut curr_j = 0;
        for _i in 0..(h + w - 2) {
            if curr_i >= h - 1 {
                if a[curr_i][curr_j + 1] == '#' {
                    curr_j += 1;
                } else {
                    println!("Impossible");
                    return;
                }
            } else if curr_j >= w - 1 {
                if a[curr_i + 1][curr_j] == '#' {
                    curr_i += 1;
                } else {
                    println!("Impossible");
                    return;
                }
            } else {
                if a[curr_i + 1][curr_j] == '#' {
                    curr_i += 1;
                } else if a[curr_i][curr_j + 1] == '#' {
                    curr_j += 1;
                } else {
                    println!("Impossible");
                    return;
                }
            }
        }
    }
    println!("Possible");
}
