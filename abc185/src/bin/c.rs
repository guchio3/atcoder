#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        l: usize
    }
    let mut use_warus = vec![11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut used_warus = vec![];
    let warus = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut res = 1;
    for i in 0..11 {
        let mut kakeru = l - i - 1;
        for &waru in use_warus.iter() {
            if kakeru % waru == 0 {
                kakeru /= waru;
                used_warus.push(waru);
            }
        }
        // println!("{}", kakeru);
        use_warus = vec![];
        for &waru in warus.iter() {
            if !used_warus.contains(&waru) {
                use_warus.push(waru);
            }
        }
        res *= kakeru;
    }
    for &waru in use_warus.iter() {
        res /= waru;
    }

    println!("{}", res as usize);
}
