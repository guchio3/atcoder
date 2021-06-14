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
        n: usize,
        a: [usize; n]
    }
    let mod_a: Vec<usize> = a.into_iter().map(|x| x % 200).collect();
    let mut cnts = HashMap::new();
    for mod_a_i in mod_a {
        *cnts.entry(mod_a_i).or_insert(0) += 1;
    }
    let mut res: usize = 0;
    for (_, value) in cnts {
        res += value * (value - 1) / 2;
    }
    println!("{}", res);
}
