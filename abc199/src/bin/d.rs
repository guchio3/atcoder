#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn dfs(
    n: usize,
    node: usize,
    colors: &mut Vec<usize>,
    edge: &Vec<Vec<bool>>,
    accessed: &mut Vec<bool>,
    tmp_accessed: &mut Vec<bool>,
) -> usize {
    accessed[node] = true;
    tmp_accessed[node] = true;

    let mut used_colors = vec![false; 4];
    for i in 0..n {
        if edge[node][i] {
            used_colors[colors[i]] = true;
        }
    }

    let mut res = 0;
    let mut color_cnt = 0;
    let mut search_cnt = 0;
    for i in 1..=3 {
        colors[node] = i;
        if !used_colors[i] {
            let mut tmptmp_accessed = vec![false; n];
            color_cnt += 1;
            let mut tmp_res = 1;
            for j in 0..n {
                if edge[node][j] && colors[j] == 0 && !tmptmp_accessed[j] {
                    search_cnt += 1;
                    tmp_res *= dfs(n, j, colors, edge, accessed, &mut tmptmp_accessed);
                }
            }
            res += tmp_res;
            for i in 0..n {
                if tmptmp_accessed[i] {
                    tmp_accessed[i] = true;
                }
            }
        }
        colors[node] = 0;
    }
    if search_cnt == 0 && color_cnt != 0 {
        color_cnt
    } else {
        res
    }
}

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m]
    }
    let mut edge = vec![vec![false; n]; n];
    for (a_i, b_i) in ab {
        edge[a_i - 1][b_i - 1] = true;
        edge[b_i - 1][a_i - 1] = true;
    }

    let mut res = 1;
    let mut accessed = vec![false; n];
    let mut tmptmp_accessed = vec![false; n];
    for i in 0..n {
        if !accessed[i] {
            let mut colors = vec![0; n];
            res *= dfs(
                n,
                i,
                &mut colors,
                &edge,
                &mut accessed,
                &mut tmptmp_accessed,
            );
        }
    }

    println!("{}", res);
}

// fn main() {
//     input! {
//         n: usize, m: usize,
//         ab: [(usize, usize); m]
//     }
//     let mut edge = vec![vec![false; n]; n];
//     for (a_i, b_i) in ab {
//         edge[a_i - 1][b_i - 1] = true;
//         edge[b_i - 1][a_i - 1] = true;
//     }
//
//     let mut used = vec![false; n];
//     let mut deque = VecDeque::new();
//     let mut res: usize = 1;
//     for i in 0..n {
//         if !used[i] {
//             deque.push_back(i);
//         } else {
//             continue;
//         }
//
//         while deque.len() > 0 {
//             let node = deque.pop_front().unwrap();
//             if used[node] {
//                 continue;
//             } else {
//                 used[node] = true;
//             }
//             let mut used_cnts = 0;
//             for i in 0..n {
//                 if edge[node][i] {
//                     if !used[i] {
//                         deque.push_back(i);
//                     } else {
//                         used_cnts += 1;
//                     }
//                 }
//             }
//             if used_cnts >= 3 {
//                 println!("0");
//                 return;
//             } else if used_cnts == 0 {
//                 res *= 3;
//             } else if used_cnts == 1 {
//                 res *= 2;
//             } else if used_cnts == 2 {
//                 res *= 2;
//             }
//         }
//     }
//     println!("{}", res);
// }
