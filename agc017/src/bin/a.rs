#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn ncr(n: usize, mut r: usize) -> usize {
    if r > n / 2 {
        r = n - r;
    }
    let mut res = 1;
    let start = max(n - r + 1, r);
    let mut r_vec: Vec<usize> = (1..=r).into_iter().collect();
    for i in start..=n {
        res *= i;
        let mut non_used = vec![];
        for &r_vec_i in r_vec.iter() {
            if res % r_vec_i == 0 {
                res /= r_vec_i;
            } else {
                non_used.push(r_vec_i);
            }
        }
        r_vec = non_used;
    }
    res
}

fn main() {
    input! {
        n: usize, p: usize,
        a: [usize; n]
    }
    let mut odd_num = 0;
    for i in 0..n {
        if a[i] % 2 == 1 {
            odd_num += 1;
        }
    }
    let even_num = n - odd_num;

    let mut res = 0;
    for i in 0..=odd_num {
        if i % 2 == p {
            res += ncr(odd_num, i);
        }
    }
    res *= 2.pow(even_num as u32);
    println!("{}", res);
}
