#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

const MOD: usize = 1_000_000_007;

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
        n: usize, a: usize, b: usize
    }
    let mut all_ncr = mod_pow(2, n, MOD);
    if all_ncr == 0 {
        all_ncr = MOD - 1;
    } else {
        all_ncr -= 1;
    }
    let nca_mod = mod_ncr(n, a, MOD);
    if nca_mod > all_ncr {
        all_ncr += MOD;
    }
    all_ncr -= nca_mod;
    let ncb_mod = mod_ncr(n, b, MOD);
    if ncb_mod > all_ncr {
        all_ncr += MOD;
    }
    all_ncr -= ncb_mod;
    println!("{}", all_ncr);
}
