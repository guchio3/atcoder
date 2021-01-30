#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

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
        mut n: i64
    }
    n *= 2;
    let mut res = 0;
    let mut divisors = get_divisors(n);
    divisors.sort();
    for divisor_i in divisors {
        if divisor_i > n / 2 {
            break;
        }
        if divisor_i % 2 == 1 {
            res += 2;
        }
    }
    println!("{}", res);
}
