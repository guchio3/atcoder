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
        n: usize, m: usize
    }
    let mut divisors = get_divisors(m);
    divisors.sort_by(|x, y| y.partial_cmp(&x).unwrap());
    for divisor_i in divisors {
        if m / divisor_i >= n {
            println!("{}", divisor_i);
            return;
        }
    }
}

// fn main() {
//     input! {
//         n: usize, m: usize
//     }
//     for i in (1..=m / n).rev() {
//         if (m - i * n) % i == 0 {
//             println!("{}", i);
//             return;
//         }
//     }
// }
