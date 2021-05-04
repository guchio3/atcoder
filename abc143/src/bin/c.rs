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
        _n: usize,
        s: Chars
    }
    let mut res = 0;
    let mut bef_char = ' ';
    for s_i in s {
        if s_i != bef_char {
            res += 1;
        }
        bef_char = s_i;
    }
    println!("{}", res);
}
