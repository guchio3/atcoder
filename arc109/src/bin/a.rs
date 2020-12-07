#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        a: i64, b: i64, x: i64, y: i64
    }
    let diff = a - b;
    if diff < 0 {
        if y <= x * 2 {
            println!("{}", diff.abs() * y + x);
        } else {
            println!("{}", diff.abs() * 2 * x + x);
        }
    } else if diff > 0 {
        if y <= x {
            println!("{}", (diff - 1) * y + x);
        } else {
            if 2 * x > y {
                println!("{}", (diff - 1) * y + x);
            } else {
                println!("{}", (diff - 1) * 2 * x + x);
            }
        }
    } else {
        println!("{}", x);
    }
}
