#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn get_prime_factors(num: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut iter_num = num;
    for i in 2..=num {
        if i > iter_num {
            break;
        } else if i * i > num {
            res.push(iter_num);
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
        x: [i64; n]
    }
    let mut prime_factors = vec![];
    let mut all_prime_factors = HashSet::new();
    for x_i in x {
        let prime_factors_i: HashSet<i64> = get_prime_factors(x_i).into_iter().collect();
        all_prime_factors = all_prime_factors
            .union(&prime_factors_i)
            .map(|x| *x)
            .collect();
        prime_factors.push(prime_factors_i);
    }

    let mut res = all_prime_factors.iter().fold(1, |a, x| a * x);
    let all_prime_factors: Vec<i64> = all_prime_factors.into_iter().collect();
    'outer: for i in 0..1 << all_prime_factors.len() {
        let mut tmp_res = 1;
        let mut tmp_prime_factors = HashSet::new();
        for j in 0..all_prime_factors.len() {
            if i & 1 << j > 0 {
                tmp_res *= all_prime_factors[j];
                tmp_prime_factors.insert(all_prime_factors[j]);
            }
        }
        for k in 0..n {
            if prime_factors[k].is_disjoint(&tmp_prime_factors) {
                continue 'outer;
            }
        }
        res = min(res, tmp_res);
    }

    println!("{}", res);
}
