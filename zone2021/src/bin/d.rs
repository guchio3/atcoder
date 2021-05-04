#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use ordered_float::NotNan;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        s: Chars
    }
    let mut res = VecDeque::new();
    let mut flipped = false;
    for i in 0..s.len() {
        let s_i = s[i];
        if s_i == 'R' {
            flipped = !flipped;
        } else {
            if flipped {
                res.push_front(s_i);
            } else {
                res.push_back(s_i);
            }
        }
    }
    if flipped {
        res = res.into_iter().rev().collect();
    }

    let mut final_res = vec![];
    for res_i in res {
        final_res.push(res_i);
        while final_res.len() > 1
            && final_res[final_res.len() - 1] == final_res[final_res.len() - 2]
        {
            final_res.pop();
            final_res.pop();
        }
    }
    println!("{}", final_res.into_iter().join(""));
}
