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
        a: [usize; n]
    }
    let mut res = 1;
    let mut odd_num = 1;
    for a_i in a {
        res *= 3;
        if a_i % 2 == 0 {
            odd_num *= 2;
        } else {
            odd_num *= 1;
        }
    }
    res -= odd_num;
    println!("{}", res);
}
