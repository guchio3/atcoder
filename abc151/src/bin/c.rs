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
        n: usize, m: usize,
        ps: [(usize, String); m]
    }

    let mut accepted = vec![false; n];
    let mut ac_num = 0;
    let mut wa_nums = vec![0; n];
    for (p_i, s_i) in ps {
        if !accepted[p_i - 1] {
            if s_i == "AC" {
                ac_num += 1;
                accepted[p_i - 1] = true;
            } else {
                wa_nums[p_i - 1] += 1;
            }
        }
    }
    let mut wa_num = 0;
    for i in 0..n {
        if accepted[i] {
            wa_num += wa_nums[i];
        }
    }
    println!("{} {}", ac_num, wa_num);
}
