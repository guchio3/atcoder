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

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut res = 0;
    for digit in 0..60 {
        let digit_template = 1usize << digit;
        let mut cnt = 0;
        for &a_i in a.iter() {
            if (digit_template & a_i) > 0 {
                cnt += 1;
            }
        }
        res = (res + (digit_template % MOD) * ((cnt * (n - cnt)) % MOD)) % MOD;
    }
    println!("{}", res);
}
