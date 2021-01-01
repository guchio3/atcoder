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
        a: [usize; n],
    }
    let mut cnt_dict = HashMap::new();
    for a_i in a {
        *cnt_dict.entry(a_i).or_insert(0) += 1;
    }
    let mut res = 1;
    for i in ((n % 2 != 0) as usize)..(n + 1) / 2 {
        if let Some(&cnt) = cnt_dict.get(&(2 * i + (n % 2 == 0) as usize)) {
            if cnt != 2 {
                println!("0");
                return;
            } else {
                res = res * 2 % MOD;
            }
        } else {
            println!("0");
            return;
        }
    }
    if n % 2 != 0 {
        if let Some(&zero_cnt) = cnt_dict.get(&0) {
            if zero_cnt != 1 {
                println!("0");
                return;
            }
        } else {
            println!("0");
            return;
        }
    }
    println!("{}", res);
}
