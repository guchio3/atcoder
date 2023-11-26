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
        n: usize, l: usize, r: usize,
        a: [usize; n]
    }

    let mut res = vec![];
    for a_i in a {
        if a_i < l {
            res.push(l);
        } else if r < a_i {
            res.push(r);
        } else {
            res.push(a_i);
        }
    }

    println!("{}", res.into_iter().join(" "));
}
