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
    if s[0] != 'A' {
        println!("WA");
        return;
    }
    let mut c_cnt = 0;
    for i in 2..s.len() - 1 {
        if s[i] == 'C' {
            c_cnt += 1;
        }
    }
    if c_cnt != 1 {
        println!("WA");
        return;
    }
    let mut small_cnt = 0;
    for i in 0..s.len() {
        if s[i] as usize >= 'a' as usize {
            small_cnt += 1;
        }
    }
    if small_cnt == s.len() - 2 {
        println!("AC");
    } else {
        println!("WA");
    }
}
