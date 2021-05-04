#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
    }
    let mut res: usize = 0;
    let mut used = vec![false; 1_000_000_000];
    for i in 2..=n {
        if i * i > n {
            break;
        }
        if !used[i] {
            let mut j: usize = i * i;
            while n >= j {
                res += 1;
                if j < 1_000_000_000 {
                    used[j] = true;
                }
                j *= i;
            }
        }
    }
    println!("{}", n - res);
}
