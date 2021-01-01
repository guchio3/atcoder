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

fn get_prime_factors(num: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut iter_num = num;
    for i in 2..=num {
        if i > iter_num {
            break;
        }
        while iter_num % i == 0 {
            res.push(i);
            iter_num /= i;
        }
    }
    res
}

fn main() {
    input! {
        n: usize,
    }

    let mut factors_dict = HashMap::new();
    for i in 1..=n {
        let a_i_prime_factors = get_prime_factors(i);
        for a_i_prime_factor_i in a_i_prime_factors {
            *factors_dict.entry(a_i_prime_factor_i).or_insert(0) += 1;
        }
    }

    let mut res = 1;
    for (key, value) in factors_dict {
        if key != 1 {
            res = res * (value + 1) % MOD;
        }
    }
    println!("{}", res);
}
