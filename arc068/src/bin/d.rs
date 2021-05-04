#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    let mut bh = BinaryHeap::new();
    let mut cnts = HashMap::new();
    for a_i in a {
        *cnts.entry(a_i).or_insert(0) += 1;
    }
    for (key, value) in cnts {
        bh.push((value, key));
    }

    while bh.peek().unwrap().0 > 1 {
        let top_pair = bh.pop().unwrap();
        if top_pair.0 == 2 {
            let second_pair = bh.pop().unwrap();
            bh.push((top_pair.0 - 1, top_pair.1));
            bh.push((second_pair.0 - 1, second_pair.1));
        } else {
            bh.push((top_pair.0 - 2, top_pair.1));
        }
        // let mut bh_pairs: Vec<(usize, usize)> = bh.iter().map(|x| *x).collect();
        // bh_pairs.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
        // println!("{:?}", bh_pairs);
    }

    let mut res = 0;
    while bh.len() > 0 {
        let top_pair = bh.pop().unwrap();
        if top_pair.0 == 1 {
            res += 1;
        }
    }

    println!("{}", res);
}
