#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

// from abc156_d
fn mod_pow(mut x: usize, mut n: usize, mod_num: usize) -> usize {
    let mut res = 1;
    while n > 0 {
        if n & 1 > 0 {
            res = (res * x) % mod_num;
        }
        x = (x * x) % mod_num;
        n >>= 1;
    }
    res
}

// from abc156_d
fn mod_ncr(n: usize, mut r: usize, mod_num: usize) -> usize {
    if r > n / 2 {
        r = n - r;
    }
    let mut res = 1;
    for i in n - r + 1..=n {
        res = (res * i) % mod_num;
    }
    let mut rev_mod = 1;
    for i in 1..=r {
        rev_mod = (rev_mod * i) % mod_num;
    }
    rev_mod = mod_pow(rev_mod, mod_num - 2, mod_num);
    res = (res * rev_mod) % mod_num;
    res
}

fn main() {
    input! {
        x: usize, y: usize
    }
    let xy_diff = max(x, y) - min(x, y);
    let jump_num;
    if xy_diff > min(x, y) || (min(x, y) - xy_diff) % 3 != 0 {
        println!("0");
        return;
    } else {
        jump_num = (min(x, y) - xy_diff) / 3;
    }
    println!(
        "{}",
        mod_ncr(jump_num * 2 + xy_diff, jump_num, 1_000_000_007)
    );
}
