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
        l: usize, r: usize
    }
    let mut res = 2019;
    for i in l..r {
        for j in i + 1..=r {
            let mod_value = (i * j) % 2019;
            if res > mod_value {
                res = mod_value;
            }
            if res == 0 {
                println!("0");
                return;
            }
        }
    }
    println!("{}", res)
}
