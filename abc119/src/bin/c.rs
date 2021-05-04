#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn calc_best_mp_cosumption(l1: i64, l2: i64, l3: i64, a: i64, b: i64, c: i64) -> i64 {
    let mut res = 1_000_000;
    for ll in vec![l1, l2, l3].into_iter().permutations(3) {
        let ll1 = ll[0];
        let ll2 = ll[1];
        let ll3 = ll[2];
        let tmp_res = (a - ll1).abs() + (b - ll2).abs() + (c - ll3).abs();
        res = min(res, tmp_res);
    }
    res
}

fn main() {
    input! {
        n: usize, a: i64, b: i64, c: i64,
        l: [i64; n]
    }
    let mut res = 1_000_000_000;
    let mut deque: VecDeque<Vec<usize>> = VecDeque::new();
    deque.push_back(vec![]);
    'outer: while deque.len() > 0 {
        let new_vec = deque.pop_front().unwrap();
        if new_vec.len() == n {
            let mut merge_num = 0;
            let mut sticks = vec![0; 4];
            for i in 0..n {
                if new_vec[i] != 0 {
                    merge_num += 1;
                }
                sticks[new_vec[i]] += l[i];
            }
            for i in 1..4 {
                if sticks[i] == 0 {
                    continue 'outer;
                }
            }
            let tmp_res = 10 * (merge_num - 3)
                + calc_best_mp_cosumption(sticks[1], sticks[2], sticks[3], a, b, c);
            res = min(res, tmp_res);
        } else {
            for i in 0..4 {
                let mut newnew_vec = new_vec.clone();
                newnew_vec.push(i);
                deque.push_back(newnew_vec);
            }
        }
    }
    println!("{}", res);
}
