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
        a: String, b: String
    }
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_num = a_chars.into_iter().fold(0, |sum, x| sum + x as u32 - '0' as u32);
    let b_num = b_chars.into_iter().fold(0, |sum, x| sum + x as u32 - '0' as u32);
    println!("{}", max(b_num, a_num));
}
