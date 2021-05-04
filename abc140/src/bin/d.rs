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
        n: usize, mut k: usize,
        mut s: Chars
    }
    let mut res = 0;
    let mut bef_s_i = 'A';
    let mut is_flipping = false;
    let mut r_met = false;
    let mut r_num = 0;
    for i in 0..n {
        if s[i] == 'L' {
            if r_met && k > 0 {
                is_flipping = true;
                s[i] = 'R';
            }
        } else {
            r_num += 1;
            r_met = true;
            if is_flipping {
                k -= 1;
            }
            is_flipping = false;
        }
        if s[i] == bef_s_i {
            res += 1;
        }
        bef_s_i = s[i];
    }
    if k > 0 && s[0] == 'L' {
        res += 1;
    }
    if r_num == 0 || r_num == n {
        println!("{}", n - 1);
    } else {
        println!("{}", res);
    }
}
