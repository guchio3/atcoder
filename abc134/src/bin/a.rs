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
        a: [i64; n]
    }

    let mut factors_dict = HashMap::new();
    for a_i in a {
        let a_i_prime_factors = get_prime_factors(a_i);
        for a_i_prime_factor_i in a_i_prime_factors {
            *factors_dict.entry(a_i_prime_factor_i).or_insert(0) += 1;
        }
    }

    let mut res;

    if reses.len() == 0 {
        println!("-1");
    } else {
        println!("{}", reses.len());
        println!("{}", reses.iter().join(" "));
    }
}
