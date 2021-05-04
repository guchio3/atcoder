#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use ordered_float::NotNan;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn dfs(n: usize, left_cnt: usize, right_cnt: usize, cur: &mut Vec<char>) {
    if cur.len() == n {
        println!("{}", cur.into_iter().join(""));
    } else {
        if left_cnt < n / 2 {
            cur.push('(');
            dfs(n, left_cnt + 1, right_cnt, cur);
            cur.pop();
        }
        if right_cnt < left_cnt {
            cur.push(')');
            dfs(n, left_cnt, right_cnt + 1, cur);
            cur.pop();
        }
    }
}

fn main() {
    input! {
        n: usize
    }

    if n % 2 == 0 {
        dfs(n, 0, 0, &mut vec![]);
    }
}
