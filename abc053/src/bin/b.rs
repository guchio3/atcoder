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
    let mut tmp_cnt = -1;
    let mut cnt = 0;
    for s_i in s {
        if s_i == 'A' && tmp_cnt < 0 {
            tmp_cnt += 1;
        }
        if tmp_cnt >= 0 {
            tmp_cnt += 1;
        }
        if s_i == 'Z' {
            cnt = tmp_cnt;
        }
    }
    println!("{}", cnt);
}
