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
        n: usize, m: usize,
        s: [Chars; n]
    }
    let s_first = s[0].clone();
    let mut diff_cnts = vec![0; 1];
    for i in 1..n {
        let s_i = s[i].clone();
        let mut diff_cnt = 0;
        for j in 0..m {
            let n_first_j = s_first[j];
            let s_i_j = s_i[j];
            if n_first_j != s_i_j {
                diff_cnt += 1;
            }
        }
        diff_cnts.push(diff_cnt);
    }

    let mut even_cnt: usize = 0;
    let mut odd_cnt: usize = 0;
    for diff_cnt in diff_cnts {
        if diff_cnt % 2 == 0 {
            even_cnt += 1;
        } else {
            odd_cnt += 1;
        }
    }
    println!("{}", even_cnt * odd_cnt);
}
