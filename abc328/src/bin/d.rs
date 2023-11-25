#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        s: Chars
    }

    let mut stack = vec![];
    for s_i in s {
        stack.push(s_i);
        while stack.len() >= 3
            && stack[stack.len() - 1] == 'C'
            && stack[stack.len() - 2] == 'B'
            && stack[stack.len() - 3] == 'A'
        {
            stack.pop();
            stack.pop();
            stack.pop();
        }
    }
    print!("{}", stack.into_iter().join(""));
}
