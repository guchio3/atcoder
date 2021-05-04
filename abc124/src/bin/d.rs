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
        n: usize, k: usize,
        s: Chars
    }
    let mut nums = vec![];
    let mut cum = 1;
    let mut bef_char = s[0];
    for i in 1..n {
        let s_i = s[i];
        if s_i != bef_char {
            nums.push(cum);
            cum = 0;
        }
        cum += 1;
        bef_char = s_i;
    }
    nums.push(cum);

    let mut res = 0;
    let mut tmp_nums = VecDeque::new();
    if s[0] == '0' {
        tmp_nums.push_back(0);
    }
    let mut first_end = min(k * 2 + (s[0] == '1') as usize, nums.len());
    for i in 0..first_end {
        res += nums[i];
        tmp_nums.push_back(nums[i]);
    }

    let mut tmp_res = res;
    loop {
        if first_end < nums.len() {
            tmp_res += nums[first_end];
            tmp_nums.push_back(nums[first_end]);
            first_end += 1;
            if first_end < nums.len() {
                tmp_res += nums[first_end];
                tmp_nums.push_back(nums[first_end]);
                first_end += 1;
            }
            tmp_res -= tmp_nums.pop_front().unwrap();
            tmp_res -= tmp_nums.pop_front().unwrap();
            res = max(res, tmp_res);
        } else {
            break;
        }
    }

    println!("{}", res);
}
