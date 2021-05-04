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
    for i in 0..s.len() {
        let s_i = s[i];
        if (i + 1) % 2 == 0 {
            // if (s_i as u8 - 'a' as u8) <= 26 {
            if s_i as u16 > 'Z' as u16 {
                println!("No");
                return;
            }
        } else {
            // if (s_i as u8 - 'a' as u8) > 26 {
            if s_i as u16 <= 'Z' as u16 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
