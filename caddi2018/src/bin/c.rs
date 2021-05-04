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
        n: i64, p: i64
    }
    let prime_factors = get_prime_factors(p);
    let mut prime_factors_cnt = HashMap::new();
    for prime_factor_i in prime_factors {
        *prime_factors_cnt.entry(prime_factor_i).or_insert(0) += 1;
    }
    let mut res: i64 = 1;
    for (key, value) in prime_factors_cnt {
        res *= key.pow((value / n) as u32);
    }
    println!("{}", res);
}
