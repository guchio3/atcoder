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
        h: usize, w: usize, m: usize,
        hw: [(usize, usize); m]
    }
    let mut hw_set = HashSet::new();
    for i in 0..m {
        hw_set.insert((hw[i].0 - 1, hw[i].1 - 1));
    }

    let mut h_cnts = vec![0; h];
    let mut w_cnts = vec![0; w];
    for i in 0..m {
        let hw_i = hw[i];
        h_cnts[hw_i.0 - 1] += 1;
        w_cnts[hw_i.1 - 1] += 1;
    }

    let mut h_max = 0;
    for i in 0..h {
        h_max = max(h_max, h_cnts[i]);
    }
    let mut h_max_places = vec![];
    for i in 0..h {
        if h_cnts[i] == h_max {
            h_max_places.push(i);
        }
    }

    let mut w_max = 0;
    for i in 0..w {
        w_max = max(w_max, w_cnts[i]);
    }
    let mut w_max_places = vec![];
    for i in 0..w {
        if w_cnts[i] == w_max {
            w_max_places.push(i);
        }
    }

    let mut non_bomb = false;
    for h_max_place in h_max_places {
        for &w_max_place in w_max_places.iter() {
            if !hw_set.contains(&(h_max_place, w_max_place)) {
                non_bomb = true;
                break;
            }
        }
    }

    if non_bomb {
        println!("{}", h_max + w_max);
    } else {
        println!("{}", h_max + w_max - 1);
    }
}
