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
        s: Chars
    }
    let set: HashSet<char> = s.into_iter().collect();
    for i in 0..26 {
        let target_char = ('a' as u8 + i) as char;
        if !set.contains(&target_char) {
            println!("{}", target_char);
            return;
        }
    }
    println!("None");
}
