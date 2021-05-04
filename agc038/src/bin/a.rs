#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, a: usize, b: usize
    }
    let mut cnts = vec![h - b; w];
    let mut res = vec![vec![1; w]; h];

    for i in 0..h {
        let mut cnt = w - a;
        for j in 0..w {
            if cnt > 0 && cnts[j] != 0 {
                cnts[j] -= 1;
                res[i][j] = 0;
                cnt -= 1;
            }
        }
    }

    // validation
    let mut col_cnts = vec![0; w];
    let mut row_cnts = vec![0; h];
    for i in 0..h {
        for j in 0..w {
            row_cnts[i] += res[i][j];
            col_cnts[j] += res[i][j];
        }
    }
    for row_cnt in row_cnts {
        if !(row_cnt == a || row_cnt == w - a) {
            println!("No");
            return;
        }
    }
    for col_cnt in col_cnts {
        if !(col_cnt == b || col_cnt == h - b) {
            println!("No");
            return;
        }
    }

    for res_i in res {
        println!("{}", res_i.into_iter().join(""));
    }
}
