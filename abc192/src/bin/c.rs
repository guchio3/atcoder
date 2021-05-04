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
        k: usize,
    }
    let mut res: Vec<char> = n.to_string().chars().collect();
    for _i in 0..k {
        res.sort_by(|x, y| y.partial_cmp(&x).unwrap());
        let g1 = res.iter().map(|x| *x).join("").parse::<usize>().unwrap();
        res.sort();
        let g2 = res.iter().map(|x| *x).join("").parse::<usize>().unwrap();
        res = (g1 - g2).to_string().chars().collect();
    }
    println!("{}", res.into_iter().join(""));
}
