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
        a: usize, b: usize, c: usize
    }
    if a == b {
        if b != c {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if b == c {
        println!("Yes");
    } else if a == c {
        println!("Yes");
    } else {
        println!("No");
    }
}
