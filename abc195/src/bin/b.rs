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
        a: f64, b: f64, w: f64
    }
    let wg = w * 1000.;
    let min_width = wg / a;
    let min_width_floor = min_width.floor();
    if wg / min_width_floor > b {
        println!("UNSATISFIABLE");
        return;
    }
    let max_width = wg / b;
    let max_width_ceil = max_width.ceil();
    if wg / max_width_ceil < a {
        println!("UNSATISFIABLE");
        return;
    }
    println!("{} {}", max_width_ceil as i64, min_width_floor as i64);

    // let wg = w * 1000;
    // let b_a_diff = b - a;
    // let mut a_num = wg / a;
    // let mut a_rest = wg % a;
    // let mut b_num = 0;
    // let mut b_num_diff = 0;
    // for i in 0..a_num {
    //     if a_rest > b_a_diff {
    //         a_rest -= b_a_diff;
    //         b_num += 1;
    //     } else {
    //         b_num_diff = b_a_diff - a_rest;
    //         a_rest = 0;
    //     }
    // }
    // if a_rest != 0 {
    //     println!("UNSATISFIABLE");
    //     return;
    // }
    // let max_num = a_num;

    // println!("{} {}", max_num, min_num);
}
