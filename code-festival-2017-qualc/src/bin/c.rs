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
    let mut left_i = 0;
    let mut right_i = s.len() - 1;
    while left_i < right_i {
        if s[left_i] == s[right_i] {
            left_i += 1;
            right_i -= 1;
        } else {
            if s[left_i] == 'x' {
                left_i += 1;
            } else if s[right_i] == 'x' {
                right_i -= 1;
            } else {
                println!("-1");
                return;
            }
            res += 1;
        }
    }
    println!("{}", res);
}
