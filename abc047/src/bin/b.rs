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
        w: usize, h: usize, n: usize,
        xya: [(usize, usize, usize); n]
    }
    let mut x_left_max = 0;
    let mut x_right_min = w;
    let mut y_bottom_max = 0;
    let mut y_upper_min = h;
    for (x_i, y_i, a_i) in xya {
        match a_i {
            1 => x_left_max = max(x_left_max, x_i),
            2 => x_right_min = min(x_right_min, x_i),
            3 => y_bottom_max = max(y_bottom_max, y_i),
            4 => y_upper_min = min(y_upper_min, y_i),
            _ => {}
        }
    }
    if x_left_max > x_right_min || y_bottom_max > y_upper_min {
        println!("0");
    } else {
        println!(
            "{}",
            (x_right_min - x_left_max) * (y_upper_min - y_bottom_max)
        );
    }
}
