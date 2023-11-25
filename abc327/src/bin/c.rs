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
        a: [[usize; 9]; 9]
    }
    for i in 0..9 {
        let mut set: HashSet<usize> = HashSet::new();
        for j in 0..9 {
            if set.contains(&a[i][j]) {
                println!("No");
                return;
            }
            set.insert(a[i][j]);
        }
    }
    for j in 0..9 {
        let mut set: HashSet<usize> = HashSet::new();
        for i in 0..9 {
            if set.contains(&a[i][j]) {
                println!("No");
                return;
            }
            set.insert(a[i][j]);
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            let mut set: HashSet<usize> = HashSet::new();
            for k in 0..9 {
                if set.contains(&a[i * 3 + k / 3][j * 3 + k % 3]) {
                    println!("No");
                    return;
                }
                set.insert(a[i * 3 + k / 3][j * 3 + k % 3]);
            }
        }
    }
    println!("Yes");
}
