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
    let mut cnts = vec![0; 4];
    for s_i in s {
        match s_i {
            'S' => cnts[0] += 1,
            'N' => cnts[1] += 1,
            'W' => cnts[2] += 1,
            'E' => cnts[3] += 1,
            _ => panic!(),
        }
    }
    let mut res1 = 0;
    for i in 0..2 {
        if cnts[i] > 0 {
            if i % 2 == 0 {
                res1 -= 1;
            } else {
                res1 += 1;
            }
        }
    }
    let mut res2 = 0;
    for i in 2..4 {
        if cnts[i] > 0 {
            if i % 2 == 0 {
                res2 -= 1;
            } else {
                res2 += 1;
            }
        }
    }

    if res1 == 0 && res2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
