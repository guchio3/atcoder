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
        mut a: usize, mut b: usize, mut c: usize,
    }
    let mut happened = HashSet::<(usize, usize, usize)>::new();
    let mut cnt = 0;
    while a % 2 == 0 && b % 2 == 0 && c % 2 == 0 {
        if happened.contains(&(a, b, c)) {
            println!("-1");
            return;
        } else {
            cnt += 1;
            happened.insert((a, b, c));
            let a_half = a / 2;
            let b_half = b / 2;
            let c_half = c / 2;
            a = b_half + c_half;
            b = a_half + c_half;
            c = a_half + b_half;
        }
    }
    println!("{}", cnt);
}
