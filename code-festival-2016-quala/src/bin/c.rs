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
        s: Chars,
        mut k: usize
    }
    let mut s_nums = vec![];
    for &s_i in s.iter() {
        s_nums.push(s_i as usize - 'a' as usize);
    }
    for i in 0..s.len() {
        let s_num = s_nums[i];
        let diff_num = 26 - s_num;
        if s_num != 0 && diff_num <= k {
            s_nums[i] = 0;
            k -= diff_num;
        }
    }
    if k > 0 {
        s_nums[s.len() - 1] = (s_nums[s.len() - 1] + k) % 26;
    }
    println!(
        "{}",
        s_nums
            .into_iter()
            .map(|x| (x + 'a' as usize) as u8 as char)
            .join("")
    );
}
