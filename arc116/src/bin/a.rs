#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

#[fastout]
fn main() {
    input! {
        t: usize
    }
    for _i in 0..t {
        input! {
            mut n: usize
        }
        if n % 2 == 0 {
            n /= 2;
            if n % 2 == 0 {
                println!("Even");
            } else {
                println!("Same")
            }
        } else {
            println!("Odd");
        }
    }
}
