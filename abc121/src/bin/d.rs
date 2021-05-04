#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn cum_xor(a: usize) -> usize {
    if a < 4 {
        let mut res = 0;
        for i in 0..=a {
            res ^= i;
        }
        res
    } else if a % 4 == 0 {
        a
    } else if a % 4 == 1 {
        a ^ (a - 1)
    } else if a % 4 == 2 {
        a ^ (a - 1) ^ (a - 2)
    } else {
        a ^ (a - 1) ^ (a - 2) ^ (a - 3)
    }
}

fn main() {
    input! {
        a: usize, b: usize
    }
    let a_cum_xor;
    if a == 0 {
        a_cum_xor = 0;
    } else {
        a_cum_xor = cum_xor(a - 1);
    }

    let b_cum_xor = cum_xor(b);

    println!("{}", a_cum_xor ^ b_cum_xor);
}
