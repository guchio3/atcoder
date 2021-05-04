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
        a: [usize; n]
    }
    // let mut num_cnt = HashMap::new();
    // let mut all_sum = 0;
    // for a_i in a {
    //     all_sum += a_i;
    //     *num_cnt.entry(a_i).or_insert(0) += 1;
    // }
    // let mut cnt_vec: Vec<(usize, usize)> = num_cnt
    //     .into_iter()
    //     .map(|(key, value)| (key, value))
    //     .collect();
    // cnt_vec.sort_by(|x, y| y.partial_cmp(&x).unwrap());

    // if all_sum % 2 == 0 {
    //     println!("second");
    // } else {
    //     println!("first");
    // }
    for a_i in a {
        if a_i % 2 != 0 {
            println!("first");
            return;
        }
    }
    println!("second");
}
