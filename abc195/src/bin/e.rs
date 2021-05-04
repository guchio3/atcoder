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
        n: usize,
        s: Chars,
        x: Chars
    }
    let mut states = vec![vec![false; 7]; n + 1];
    states[n][0] = true;
    for i in (0..n).rev() {
        let s_i = s[i] as usize - '0' as usize;
        let x_i = x[i];

        for j in 0..7 {
            if x_i == 'A' {
                if states[i + 1][(j * 10) % 7] && states[i + 1][(j * 10 + s_i) % 7] {
                    states[i][j] = true;
                }
            } else {
                if states[i + 1][(j * 10) % 7] || states[i + 1][(j * 10 + s_i) % 7] {
                    states[i][j] = true;
                }
            }
        }
    }
    if states[0][0] {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
