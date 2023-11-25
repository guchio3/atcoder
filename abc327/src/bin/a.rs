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
        s: Chars,
    }

    for i in 0..n - 1 {
        if s[i] == 'a' && s[i + 1] == 'b' || s[i] == 'b' && s[i + 1] == 'a' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
