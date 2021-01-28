#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn get_prime_numbers_lt(num: usize) -> Vec<usize> {
    let mut res = vec![];
    'outer: for i in 2..num {
        for j in 0..res.len() {
            if i % res[j] == 0 {
                continue 'outer;
            }
        }
        res.push(i);
    }
    res
}

fn main() {
    input! {
        q: usize,
        lr: [(usize, usize); q]
    }
    let prime_nums = get_prime_numbers_lt(100_001);
    let mut target_prime_nums = vec![];
    for &prime_num_i in prime_nums.iter() {
        if prime_num_i % 2 == 1 && prime_nums.contains(&((prime_num_i + 1) / 2)) {
            target_prime_nums.push(prime_num_i);
        }
    }

    for (l_i, r_i) in lr {
        let r_value;
        match target_prime_nums.binary_search(&r_i) {
            Ok(value) => r_value = value + 1,
            Err(value) => r_value = value,
        };
        let l_value;
        match target_prime_nums.binary_search(&l_i) {
            Ok(value) => l_value = value,
            Err(value) => l_value = value,
        };
        println!("{}", r_value - l_value);
    }
}
