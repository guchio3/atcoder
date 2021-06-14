#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use ordered_float::NotNan;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        _n: usize,
        s: Chars
    }
    let mut mod_prod_cnts = vec![0; 7];
    for s_i in s {
        match s_i {
            'a' => mod_prod_cnts[0] = (mod_prod_cnts[0] + 1) % MOD,
            't' => mod_prod_cnts[1] = (mod_prod_cnts[0] + mod_prod_cnts[1]) % MOD,
            'c' => mod_prod_cnts[2] = (mod_prod_cnts[1] + mod_prod_cnts[2]) % MOD,
            'o' => mod_prod_cnts[3] = (mod_prod_cnts[2] + mod_prod_cnts[3]) % MOD,
            'd' => mod_prod_cnts[4] = (mod_prod_cnts[3] + mod_prod_cnts[4]) % MOD,
            'e' => mod_prod_cnts[5] = (mod_prod_cnts[4] + mod_prod_cnts[5]) % MOD,
            'r' => mod_prod_cnts[6] = (mod_prod_cnts[5] + mod_prod_cnts[6]) % MOD,
            _ => {}
        }
    }
    println!("{}", mod_prod_cnts[6]);
}
