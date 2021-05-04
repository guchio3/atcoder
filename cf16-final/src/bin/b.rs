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
        n: usize
    }
    let mut sum = 0;
    let mut reses = vec![];
    let mut ng = 0;
    for i in 1..=n {
        sum += i;
        reses.push(i);
        if sum > n {
            ng = sum - n;
            break;
        }
    }
    for res in reses {
        if res != ng {
            println!("{}", res);
        }
    }
}
