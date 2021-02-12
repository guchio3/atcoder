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
        mut s: Chars
    }
    let mut s_copy = s.clone();
    if s_copy[0] == '0' {
        s_copy[0] = '1';
    } else {
        s_copy[0] = '0';
    }
    let mut res = 0;
    let mut res_copy = 1;
    for i in 1..s.len() {
        if s[i] == s[i - 1] {
            if s[i] == '0' {
                s[i] = '1';
            } else {
                s[i] = '0';
            }
            res += 1;
        }
        if s_copy[i] == s_copy[i - 1] {
            if s_copy[i] == '0' {
                s_copy[i] = '1';
            } else {
                s_copy[i] = '0';
            }
            res_copy += 1;
        }
    }
    println!("{}", min(res, res_copy));
}
