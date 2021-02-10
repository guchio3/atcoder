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
        mut sa: Chars,
        mut sb: Chars,
        mut sc: Chars,
    }
    sa = sa.into_iter().rev().collect();
    sb = sb.into_iter().rev().collect();
    sc = sc.into_iter().rev().collect();

    let mut turn = 'a';
    loop {
        if turn == 'a' {
            if let Some(next_turn) = sa.pop() {
                turn = next_turn;
            } else {
                break;
            }
        } else if turn == 'b' {
            if let Some(next_turn) = sb.pop() {
                turn = next_turn;
            } else {
                break;
            }
        } else if turn == 'c' {
            if let Some(next_turn) = sc.pop() {
                turn = next_turn;
            } else {
                break;
            }
        }
    }
    println!("{}", (turn as u8 + 'A' as u8 - 'a' as u8) as char);
}
