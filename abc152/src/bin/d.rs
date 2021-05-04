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
        n: usize
    }
    let mut res = 0;
    let mut digit = 1;
    let mut cnts = vec![vec![0; 10]; 10];
    for i in 1..=n {
        if i >= digit * 10 {
            digit *= 10;
        }
        let digi1 = i % 10;
        let digi2 = i / digit;
        cnts[digi1][digi2] += 1;
    }

    for i in 0..10 {
        for j in 0..10 {
            res += cnts[i][j] * cnts[j][i];
        }
    }

    println!("{}", res);
}
