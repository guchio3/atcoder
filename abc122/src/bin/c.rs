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
        n: usize, q: usize,
        s: Chars,
        lr: [(usize, usize); q]
    }
    let mut cum_ac = 0;
    let mut cum_acs = vec![0; n];
    let mut bef_char = ' ';
    for i in 0..n {
        let target_char = s[i];
        if bef_char == 'A' && target_char == 'C' {
            cum_ac += 1;
        }
        cum_acs[i] = cum_ac;
        bef_char = target_char;
    }
    for (li, ri) in lr {
        let res = cum_acs[ri - 1] - cum_acs[li - 1];
        println!("{}", res);
    }
}
