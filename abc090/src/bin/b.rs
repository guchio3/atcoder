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
        a: usize, b: usize
    }
    let mut res = 0;
    'outer: for i in a..=b {
        let i_str: Vec<char> = i.to_string().chars().collect();
        for j in 0..i_str.len() / 2 {
            if i_str[j] != i_str[i_str.len() - j - 1] {
                continue 'outer;
            }
        }
        res += 1;
    }
    println!("{}", res);
}
