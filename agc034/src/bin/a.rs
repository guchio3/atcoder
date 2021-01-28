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
        n: usize, a: usize, b: usize, c: usize, d: usize,
        s: Chars
    }
    // まず追い抜き可能かを考える
    let mut can_jump = false;
    let mut num = 0;
    for i in max(2, b) - 2..=min(c, d) {
        if s[i] == '.' {
            num += 1;
        } else {
            num = 0;
        }
        if num > 2 {
            can_jump = true;
            break;
        }
    }
    if !can_jump && c > d {
        println!("No");
        return;
    }

    let mut sunuke_vec = vec![false; n];
    let mut hunuke_vec = vec![false; n];
    sunuke_vec[a - 1] = true;
    hunuke_vec[b - 1] = true;
    for i in 0..n {
        let s_i = s[i];
        if s_i == '.' {
            if hunuke_vec[max(1, i) - 1] || hunuke_vec[max(2, i) - 2] {
                hunuke_vec[i] = true;
            }
        }
    }
    if !hunuke_vec[d - 1] {
        println!("No");
        return;
    }

    for i in 0..n {
        let s_i = s[i];
        if s_i == '.' {
            if sunuke_vec[max(1, i) - 1] || sunuke_vec[max(2, i) - 2] {
                sunuke_vec[i] = true;
            }
        }
    }
    if !sunuke_vec[c - 1] {
        println!("No");
    } else {
        println!("Yes");
    }
}
