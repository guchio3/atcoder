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
        n: usize,
        s: [String; n]
    }
    let mut general_cnt = vec![0; 26];

    for s_ij in s[0].chars() {
        general_cnt[s_ij as usize - 'a' as usize] += 1;
    }
    for i in 1..n {
        let s_i = s[i].chars();
        let mut tmp_cnt = vec![0; 26];
        for s_ij in s_i {
            tmp_cnt[s_ij as usize - 'a' as usize] += 1;
        }
        for j in 0..26 {
            general_cnt[j] = min(general_cnt[j], tmp_cnt[j])
        }
    }
    let mut res = vec![];
    for i in 0..26 {
        for _j in 0..general_cnt[i] {
            res.push(('a' as u8 + i as u8) as char);
        }
    }
    println!("{}", res.into_iter().join(""));
}
