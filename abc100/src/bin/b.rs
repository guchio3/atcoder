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
        d: usize, n: usize
    }
    let mut num = 1;
    for _i in 0..d {
        num *= 100;
    }
    let mut res = 0;
    for _j in 0..n {
        res += num;
        loop {
            match d {
                0 => {
                    if res % 100 == 0 {
                        res += num;
                    } else {
                        break;
                    }
                }
                1 => {
                    if res % 10000 == 0 {
                        res += num;
                    } else {
                        break;
                    }
                }
                _ => {
                    if res % 1000000 == 0 {
                        res += num;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    println!("{}", res);
}
