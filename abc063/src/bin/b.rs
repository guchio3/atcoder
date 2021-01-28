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
        s: Chars,
    }
    let mut used = vec![false; 26];
    for s_i in s {
        let index = s_i as usize - 'a' as usize;
        if used[index] {
            println!("no");
            return;
        } else {
            used[index] = true;
        }
    }
    println!("yes");
}
