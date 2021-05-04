#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn parse_nums(nums: &mut Vec<char>) -> usize {
    let mut res = 0;
    let mut digit = 1;
    while nums.len() > 0 {
        res += digit * (nums.pop().unwrap() as usize - '0' as usize);
        digit *= 10;
    }
    res
}

fn main() {
    input! {
        s: Chars
    }
    let mut res = 0;
    for i in 0..(1 << (s.len() - 1)) {
        let mut nums = vec![s[0]];
        for j in 0..(s.len() - 1) {
            if i & (1 << j) > 0 {
                res += parse_nums(&mut nums);
            }
            nums.push(s[j + 1]);
        }
        res += parse_nums(&mut nums);
    }
    println!("{}", res);
}
