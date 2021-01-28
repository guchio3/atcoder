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
    let mut res = 0;
    let mut tmp = 0;
    for s_i in s {
        if s_i == 'I' {
            tmp += 1;
        } else {
            tmp -= 1;
        }
        res = max(res, tmp);
    }
    println!("{}", res);
}
