#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn bfs(
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    s: &Vec<Vec<char>>,
) -> usize {
    let mut to_access = VecDeque::<(usize, usize, usize)>::new();
    let mut res = 0;
    let mut accessed = vec![vec![false; s[0].len()]; s.len()];
    accessed[start_y][start_x] = true;
    to_access.push_back((start_y, start_x, 0));
    while let Some(next) = to_access.pop_front() {
        if next.0 == end_y && next.1 == end_x {
            res = next.2;
            break;
        }
        if next.0 > 0 && s[next.0 - 1][next.1] == '.' && !accessed[next.0 - 1][next.1] {
            accessed[next.0 - 1][next.1] = true;
            to_access.push_back((next.0 - 1, next.1, next.2 + 1));
        }
        if next.1 > 0 && s[next.0][next.1 - 1] == '.' && !accessed[next.0][next.1 - 1] {
            accessed[next.0][next.1 - 1] = true;
            to_access.push_back((next.0, next.1 - 1, next.2 + 1));
        }
        if next.0 < s.len() - 1 && s[next.0 + 1][next.1] == '.' && !accessed[next.0 + 1][next.1] {
            accessed[next.0 + 1][next.1] = true;
            to_access.push_back((next.0 + 1, next.1, next.2 + 1));
        }
        if next.1 < s[0].len() - 1 && s[next.0][next.1 + 1] == '.' && !accessed[next.0][next.1 + 1] {
            accessed[next.0][next.1 + 1] = true;
            to_access.push_back((next.0, next.1 + 1, next.2 + 1));
        }
    }
    res
}

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h]
    }
    // bfs 全探索
    // start
    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            // goal
            for k in 0..h {
                for l in 0..w {
                    if s[k][l] == '#' {
                        continue;
                    }
                    res = max(res, bfs(j, i, l, k, &s));
                }
            }
        }
    }
    println!("{}", res);
}
