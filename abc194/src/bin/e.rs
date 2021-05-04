#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n]
    }
    let mut cnts = HashMap::new();
    let mut can_be_used = vec![true; n + 1];
    for i in 0..m {
        can_be_used[a[i]] = false;
        *cnts.entry(a[i]).or_insert(0) += 1
    }
    let mut bh = BinaryHeap::new();
    for i in 0..=n {
        if can_be_used[i] {
            bh.push(Reverse(i));
        }
    }
    let Reverse(mut res) = *bh.peek().unwrap();
    for i in 0..n - m {
        let removed = a[i];
        let added = a[i + m];
        let mut removed_cnt = *cnts.get_mut(&removed).unwrap();
        removed_cnt -= 1;
        if removed_cnt == 0 {
            can_be_used[removed] = true;
            bh.push(Reverse(removed));
        }
        *cnts.entry(added).or_insert(0) += 1;
        can_be_used[added] = false;
        loop {
            let Reverse(peek_value) = *bh.peek().unwrap();
            if can_be_used[peek_value] {
                break;
            } else {
                bh.pop();
            }
        }
        let Reverse(peek_value) = *bh.peek().unwrap();
        res = min(res, peek_value);
    }
    println!("{}", res);
}
