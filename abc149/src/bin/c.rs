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
        mut x: usize,
    }
    while get_divisors(x).len() > 2 {
        x += 1;
    }
    println!("{}", x);
}
