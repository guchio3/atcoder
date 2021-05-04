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
        ab: [(usize, usize); n]
    }
    let mut cnts = vec![0; 100_001];
    for (ai, bi) in ab {
        cnts[ai] += bi;
    }

    let mut cum_cnt = 0;
    for i in 0..cnts.len() {
        cum_cnt += cnts[i];
        if cum_cnt >= k {
            println!("{}", i);
            return;
        }
    }
}

// fn main() {
//     input! {
//         n: usize, k: usize,
//         ab: [(usize, usize); n]
//     }
//     let mut nums_cnt = HashMap::new();
//     for (ai, bi) in ab {
//         *(nums_cnt.entry(ai).or_insert(0)) += bi;
//     }
//     let mut keys: Vec<usize> = nums_cnt.keys().into_iter().map(|x| *x).collect();
//     keys.sort();
//
//     let mut i = 0;
//     let mut cum_cnt = 0;
//     while cum_cnt < k {
//         let key = keys[i];
//         cum_cnt += *nums_cnt.get(&key).unwrap();
//         i += 1;
//     }
//     println!("{}", keys[i - 1]);
// }
