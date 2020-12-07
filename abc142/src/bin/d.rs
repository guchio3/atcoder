#![allow(unused_imports)]
use itertools::Itertools;
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn get_divisors<T>(num: T) -> Vec<T>
where
    T: PrimInt + NumAssign,
{
    let mut i = T::one();
    let mut res = vec![];
    while i * i <= num {
        if num % i == T::zero() {
            let rev_i = num / i;
            res.push(i);
            if i != rev_i {
                res.push(rev_i);
            }
        }
        i += T::one();
    }
    res
}

fn main() {
    input! {
        a: u64, b: u64
    }

    let divisors_a: HashSet<u64> = get_divisors(a).into_iter().collect();
    let divisors_b: HashSet<u64> = get_divisors(b).into_iter().collect();

    let mut common_divisors: Vec<u64> = divisors_a
        .intersection(&divisors_b)
        .into_iter()
        .map(|x| *x)
        .collect();
    common_divisors.sort();

    let mut res_divisors = vec![1];
    for &divisors_i in common_divisors.iter() {
        let mut push_flg = true;
        for &divisors_j in res_divisors.iter() {
            if !(divisors_j < divisors_i && (divisors_j == 1 || divisors_i % divisors_j != 0)) {
                // println!("{} - {}", divisors_i, divisors_j);
                push_flg = false;
                break;
            }
        }
        if push_flg {
            res_divisors.push(divisors_i);
        }
    }

    println!("{}", res_divisors.len());
}
