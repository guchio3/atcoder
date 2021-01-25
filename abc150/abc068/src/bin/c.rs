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
        n: usize, m: usize,
        ab: [(usize, usize); m]
    }
    let mut edge_dict = HashMap::new();
    for (a_i, b_i) in ab {
        (*edge_dict.entry(a_i - 1).or_insert(vec![])).push(b_i - 1);
        (*edge_dict.entry(b_i - 1).or_insert(vec![])).push(a_i - 1);
    }
    if let Some(nexts) = edge_dict.get(&0) {
        for next in nexts {
            if let Some(nextnexts) = edge_dict.get(&next) {
                if nextnexts.contains(&(n - 1)) {
                    println!("POSSIBLE");
                    return;
                }
            }
        }
    }
    println!("IMPOSSIBLE");
}

// fn dfs(
//     now: usize,
//     goal: usize,
//     edge_dict: &HashMap<usize, Vec<usize>>,
//     accessed: &mut Vec<bool>,
//     depth: usize,
// ) -> bool {
//     if now == goal {
//         return true;
//     }
//     if depth > 2 {
//         return false;
//     }
//     if let Some(next_nodes) = edge_dict.get(&now) {
//         for &node in next_nodes {
//             if !accessed[node] {
//                 accessed[node] = true;
//                 let res = dfs(node, goal, edge_dict, accessed, depth + 1);
//                 if res {
//                     return true;
//                 }
//             }
//         }
//     }
//     false
// }
//
// fn main() {
//     input! {
//         n: usize, m: usize,
//         ab: [(usize, usize); m]
//     }
//     let mut edge_dict = HashMap::new();
//     for (a_i, b_i) in ab {
//         (*edge_dict.entry(a_i - 1).or_insert(vec![])).push(b_i - 1);
//         (*edge_dict.entry(b_i - 1).or_insert(vec![])).push(a_i - 1);
//     }
//     let mut accessed = vec![false; n];
//     accessed[0] = true;
//     let res = dfs(0, n - 1, &edge_dict, &mut accessed, 1);
//     if res {
//         println!("POSSIBLE");
//     } else {
//         println!("IMPOSSIBLE");
//     }
// }
