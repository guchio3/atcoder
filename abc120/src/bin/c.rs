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
        s: Chars
    }
    let mut res = 0;
    let mut stack = vec![];
    for i in 0..s.len() {
        if stack.len() > 0 && stack[stack.len() - 1] != s[i] {
            res += 2;
            stack.pop();
        } else {
            stack.push(s[i]);
        }
    }
    println!("{}", res);
}
