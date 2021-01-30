#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        a: usize, _b: usize,
        s: Chars
    }
    let hifun_cnt: usize = s.iter().map(|x| (*x == '-') as usize).sum::<usize>();
    if s[a] == '-' && hifun_cnt == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
