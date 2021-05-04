#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

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

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize, k: usize
    }
    for i in 1..=k {
        if (n - k) < i - 1 {
            println!("0");
        } else {
            let blue_rest_num = k - i;
            let red_rest_num = (n - k) - (i - 1);
            let mut res: usize = 1;
            res = (res * mod_ncr(blue_rest_num + i - 1, i - 1, MOD)) % MOD;
            res = (res * mod_ncr(red_rest_num + i, i, MOD)) % MOD;
            println!("{}", res);
        }
    }
}
