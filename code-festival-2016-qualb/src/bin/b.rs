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
        n: usize, a: usize, b: usize,
        s: Chars
    }
    let mut passed_num = 0;
    let mut foreign_passed_num = 0;
    for i in 0..n {
        let person = s[i];
        if person == 'a' {
            if passed_num < a + b {
                println!("Yes");
                passed_num += 1;
            } else {
                println!("No");
            }
        } else if person == 'b' {
            if passed_num < a + b && foreign_passed_num < b {
                passed_num += 1;
                foreign_passed_num += 1;
                println!("Yes");
            } else {
                println!("No");
            }
        } else {
            println!("No");
        }
    }
}
