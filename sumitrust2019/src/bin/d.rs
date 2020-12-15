#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        _n: usize,
        s: Chars
    }
    let mut mem_1 = vec![false; 10];
    let mut mem_2 = vec![vec![false; 10]; 10];
    let mut mem_3 = vec![vec![vec![false; 10]; 10]; 10];
    for i in (0..s.len()).rev() {
        let s_i_usize = (s[i] as u32 - '0' as u32) as usize;
        if i < s.len() - 2 {
            for j in 0..10 {
                for k in 0..10 {
                    if mem_2[j][k] {
                        mem_3[s_i_usize][j][k] = true;
                    }
                }
            }
        }
        if i < s.len() - 1 {
            for j in 0..10 {
                if mem_1[j] {
                    mem_2[s_i_usize][j] = true;
                }
            }
        }
        mem_1[(s[i] as u32 - '0' as u32) as usize] = true;
    }

    let mut res = 0;
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                res += mem_3[i][j][k] as usize;
            }
        }
    }

    println!("{}", res);
}
