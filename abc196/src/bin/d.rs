#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn dfs(
    target: (usize, usize),
    used: &mut Vec<Vec<bool>>,
    used_num: usize,
    mut res: usize,
    h: usize,
    w: usize,
    a: usize,
) -> usize {
    if (target.0 == h - 1 && target.1 == w - 1) || target.0 > h - 1 || target.1 > w - 1 {
        return res;
    }
    for i in 0..3 {
        match i {
            0 => {
                // 横
                if target.1 + 1 <= w - 1
                    && !used[target.0][target.1]
                    && !used[target.0][target.1 + 1]
                {
                    used[target.0][target.1] = true;
                    used[target.0][target.1 + 1] = true;
                    let next_target;
                    if target.1 == w - 1 {
                        next_target = (target.0 + 1, 0);
                    } else {
                        next_target = (target.0, target.1 + 1);
                    }
                    if used_num + 1 == a {
                        // println!("{:?}, {:?}, {}", used, target, i);
                        res += 1;
                    } else {
                        res = dfs(next_target, used, used_num + 1, res, h, w, a);
                    }
                    used[target.0][target.1] = false;
                    used[target.0][target.1 + 1] = false;
                } else {
                }
            }
            1 => {
                // 縦
                if target.0 + 1 <= h - 1
                    && !used[target.0][target.1]
                    && !used[target.0 + 1][target.1]
                {
                    used[target.0][target.1] = true;
                    used[target.0 + 1][target.1] = true;
                    let next_target;
                    if target.1 == w - 1 {
                        next_target = (target.0 + 1, 0);
                    } else {
                        next_target = (target.0, target.1 + 1);
                    }
                    if used_num + 1 == a {
                        // println!("{:?}, {:?}, {}", used, target, i);
                        res += 1;
                    } else {
                        res = dfs(next_target, used, used_num + 1, res, h, w, a);
                    }
                    used[target.0][target.1] = false;
                    used[target.0 + 1][target.1] = false;
                } else {
                }
            }
            _ => {
                let next_target;
                if target.1 == w - 1 {
                    next_target = (target.0 + 1, 0);
                } else {
                    next_target = (target.0, target.1 + 1);
                }
                res = dfs(next_target, used, used_num, res, h, w, a);
            }
        }
    }
    res
}

fn main() {
    input! {
        h: usize, w: usize, a: usize, _b: usize
    }
    if a == 0 {
        println!("1");
    } else {
        let mut used = vec![vec![false; w]; h];
        let res = dfs((0, 0), &mut used, 0, 0, h, w, a);
        println!("{}", res);
    }
}
