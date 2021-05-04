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
    let mut cnts = HashMap::new();
    for s_i in s {
        *cnts.entry(s_i).or_insert(0) += 1
    }
    let mut reses = vec![];
    let mut max_cnt = 0;
    for (key, value) in cnts {
        if value > max_cnt {
            max_cnt = value;
            reses = vec![key];
        } else if value == max_cnt {
            reses.push(key);
        }
    }
    reses.sort();
    for res in reses {
        println!("{}", res);
    }
}
