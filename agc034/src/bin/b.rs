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
        s: Chars
    }
    let mut res: usize = 0;
    let mut a_cnt = 0;
    let mut phase = 0;
    for s_i in s {
        if phase == 0 {
            if s_i == 'A' {
                phase += 1;
                a_cnt += 1;
            } else if s_i == 'B' {
                phase = 0;
                a_cnt = 0;
            } else if s_i == 'C' {
                phase = 0;
                a_cnt = 0;
            } else {
                panic!();
            }
        } else if phase == 1 {
            if s_i == 'A' {
                a_cnt += 1;
            } else if s_i == 'B' {
                phase += 1;
            } else if s_i == 'C' {
                phase = 0;
                a_cnt = 0;
            } else {
                panic!();
            }
        } else if phase == 2 {
            if s_i == 'A' {
                phase = 1;
                a_cnt = 1;
            } else if s_i == 'B' {
                phase = 0;
                a_cnt = 0;
            } else if s_i == 'C' {
                res += a_cnt;
                phase = 1;
                a_cnt = a_cnt;
            } else {
                panic!();
            }
        }
    }
    println!("{}", res);
}
