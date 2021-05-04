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
        s: Chars
    }
    let mut e_num = 0;
    for &s_i in s.iter() {
        if s_i == 'E' {
            e_num += 1;
        }
    }
    let mut curr_e_num = 0;
    let mut curr_w_num = 0;
    let mut res = n;
    for s_i in s {
        if s_i == 'E' {
            res = min(res, curr_w_num + e_num - curr_e_num - 1);
            curr_e_num += 1;
        } else {
            res = min(res, curr_w_num + e_num - curr_e_num);
            curr_w_num += 1;
        }
    }
    println!("{}", res);
}
