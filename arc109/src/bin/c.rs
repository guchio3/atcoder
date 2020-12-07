#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        mut n: usize, k: usize,
        mut s: Chars
    }

    for _ in 0..k {
        if n % 2 != 0 {
            s.append(&mut s.clone());
            n = s.len();
        }
        let mut temp_s: Vec<char> = vec![];
        for j in 0..n / 2 {
            if s[j * 2] == 'R' {
                if s[j * 2 + 1] == 'R' || s[j * 2 + 1] == 'S' {
                    temp_s.push('R');
                } else {
                    temp_s.push('P');
                }
            } else if s[j * 2] == 'S' {
                if s[j * 2 + 1] == 'S' || s[j * 2 + 1] == 'P' {
                    temp_s.push('S');
                } else {
                    temp_s.push('R');
                }
            } else {
                if s[j * 2 + 1] == 'P' || s[j * 2 + 1] == 'R' {
                    temp_s.push('P');
                } else {
                    temp_s.push('S');
                }
            }
        }
        n = temp_s.len();
        s = temp_s;
        // println!("{:?}", s);
    }

    // println!("{:?}", s);
    println!("{}", s[0]);
}
