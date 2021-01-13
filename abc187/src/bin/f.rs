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
    // bitDP
    let mut graph = vec![vec![false; n]; n];
    for ab_i in ab {
        graph[ab_i.0 - 1][ab_i.1 - 1] = true;
        graph[ab_i.1 - 1][ab_i.0 - 1] = true;
    }

    let mut is_creeks = vec![true; 1 << n];
    for use_node_bits in 0..(1 << n) {
        let mut use_nodes = vec![];
        for i in 0..n {
            if use_node_bits & (1 << i) != 0 {
                use_nodes.push(i);
            }
            'creek_check: for node_i in 0..use_nodes.len() {
                for node_j in node_i + 1..use_nodes.len() {
                    // 一つでも edge がなければ not creek
                    if !graph[use_nodes[node_i]][use_nodes[node_j]] {
                        is_creeks[use_node_bits] = false;
                        break 'creek_check;
                    }
                }
            }
        }
    }

    let mut dp = vec![100000000; 1 << n];
    dp[0] = 0;
    for bit in 0..(1 << n) {
        if dp[bit] < 100000000 {
            let cbit = (1 << n) - 1 - bit;
            let mut add = cbit;
            while add > 0 {
                if is_creeks[add] {
                    dp[bit | add] = min(dp[bit | add], dp[bit] + 1);
                }
                add = (add - 1) & cbit;
            }
        }
    }

    println!("{}", dp[(1 << n) - 1]);
}

// fn main() {
//     input! {
//         n:usize, m: usize,
//         ab: [(usize, usize); m]
//     }
//     let mut edges = vec![vec![false; n]; n];
//     for ab_i in ab {
//         edges[ab_i.0 - 1][ab_i.1 - 1] = true;
//         edges[ab_i.1 - 1][ab_i.0 - 1] = true;
//     }
//     let mut res = 0;
//     let mut used = vec![false; n];
//     for i in 0..n {
//         if !used[i] {
//             let mut candidates = vec![i];
//             for j in i + 1..n {
//                 for &candidate in candidates.iter() {
//                     if !edges[j][candidate] {
//                         for &tmp_candidate in candidates.iter() {
//                             edges[j][tmp_candidate] = false;
//                             edges[tmp_candidate][j] = false;
//                         }
//                     }
//                 }
//             }
//             used[i] = true;
//             res += 1;
//         }
//     }
//
//     println!("{}", res);
// }
