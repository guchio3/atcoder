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
        a: Chars,
        b: Chars
    }
    if a.len() > b.len() {
        println!("GREATER");
        return;
    } else if a.len() < b.len() {
        println!("LESS");
        return;
    } else {
        for i in 0..a.len() {
            if a[i] > b[i] {
                println!("GREATER");
                return;
            } else if a[i] < b[i] {
                println!("LESS");
                return;
            }
        }
    }
    println!("EQUAL");
}
