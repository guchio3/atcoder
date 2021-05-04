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
        ab: [(usize, usize); m]
    }
    let mut cnts = vec![0; n];
    for (a_i, b_i) in ab {
        cnts[a_i - 1] += 1;
        cnts[b_i - 1] += 1;
    }
    if cnts.into_iter().all(|x| x % 2 == 0) {
        println!("YES");
    } else {
        println!("NO");
    }
    // let mut query_cnts = HashMap::new();
    // for i in 0..m {
    //     let (a_i, b_i) = ab[i];
    //     *query_cnts
    //         .entry((min(a_i, b_i), max(a_i, b_i)))
    //         .or_insert(0) += 1;
    // }
    // let mut querys: Vec<(usize, usize)> = query_cnts.keys().into_iter().map(|x| *x).collect();
    // querys.sort();

    // for query in querys {
    //     ;
    // }
}
