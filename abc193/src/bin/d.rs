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
        k: usize,
        s: Chars,
        t: Chars
    }
    let mut win_num = 0;
    let mut all_num = 0;
    let mut takahashi_nums = vec![0; 10];
    let mut aoki_nums = vec![0; 10];
    for i in 0..4 {
        takahashi_nums[s[i] as usize - '0' as usize] += 1;
        aoki_nums[t[i] as usize - '0' as usize] += 1;
    }
    let mut nums = vec![k; 10];
    for i in 1..=9 {
        nums[i] -= takahashi_nums[i];
        nums[i] -= aoki_nums[i];
    }
    for i in 1..=9 {
        for j in 1..=9 {
            if (i == j && nums[i] > 1) || (nums[i] > 0 && nums[j] > 0) {
                let mut takahashi_score = 0;
                let mut aoki_score = 0;
                takahashi_nums[i] += 1;
                aoki_nums[j] += 1;
                for l in 1..=9 {
                    takahashi_score += l * 10usize.pow(takahashi_nums[l] as u32);
                    aoki_score += l * 10usize.pow(aoki_nums[l] as u32);
                }
                let mut current_case = 1;
                current_case *= nums[i];
                nums[i] -= 1;
                current_case *= nums[j];
                nums[i] += 1;
                if takahashi_score > aoki_score {
                    win_num += current_case;
                }
                all_num += current_case;
                takahashi_nums[i] -= 1;
                aoki_nums[j] -= 1;
            }
        }
    }
    println!("{}", win_num as f64 / all_num as f64);
}
