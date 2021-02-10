#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

const MAX_COST: i64 = 1_000_000_000;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        abc: [(usize, usize, i64); m]
    }
    let mut self_costs = vec![MAX_COST; n];
    let mut edge = HashMap::new();
    for (a_i, b_i, c_i) in abc {
        (*edge.entry(a_i - 1).or_insert(vec![])).push((b_i - 1, c_i));
        if a_i == b_i {
            self_costs[a_i - 1] = min(self_costs[a_i - 1], c_i);
        }
    }
    // dijkstra
    for i in 0..n {
        let mut bh = BinaryHeap::new();
        bh.push(Reverse((0, i)));
        let mut forward_cost = vec![MAX_COST; n];
        // forward_cost[i] = 0;
        while let Some(Reverse(node)) = bh.pop() {
            if let Some(next_nodes) = edge.get_mut(&node.1) {
                next_nodes.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
                for next_node_cost in next_nodes {
                    let next_node = next_node_cost.0;
                    let next_cost = next_node_cost.1;
                    let cost = node.0 + next_cost;
                    if forward_cost[next_node] > cost {
                        forward_cost[next_node] = cost;
                        bh.push(Reverse((forward_cost[next_node], next_node)));
                    }
                }
            }
        }
        let min_cost = forward_cost[i];
        if min_cost == MAX_COST {
            println!("-1");
        } else {
            println!("{}", min_cost);
        }
    }
}

// #[fastout]
// fn main() {
//     input! {
//         n: usize, m: usize,
//         abc: [(usize, usize, i64); m]
//     }
//     let mut self_costs = vec![MAX_COST; n];
//     let mut edge = HashMap::new();
//     let mut rev_edge = HashMap::new();
//     for (a_i, b_i, c_i) in abc {
//         (*edge.entry(a_i - 1).or_insert(vec![])).push((b_i - 1, c_i));
//         (*rev_edge.entry(b_i - 1).or_insert(vec![])).push((a_i - 1, c_i));
//         if a_i == b_i {
//             self_costs[a_i - 1] = min(self_costs[a_i - 1], c_i);
//         }
//     }
//     // dijkstra
//     for i in 0..n {
//         // forward
//         let mut bh = BinaryHeap::new();
//         bh.push(Reverse((0, i)));
//         let mut forward_cost = vec![MAX_COST; n];
//         forward_cost[i] = 0;
//         while let Some(Reverse(node)) = bh.pop() {
//             if let Some(next_nodes) = edge.get_mut(&node.1) {
//                 next_nodes.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
//                 for next_node_cost in next_nodes {
//                     let next_node = next_node_cost.0;
//                     let next_cost = next_node_cost.1;
//                     let cost = forward_cost[node.1] + next_cost;
//                     if next_node != i && forward_cost[next_node] > cost {
//                         forward_cost[next_node] = cost;
//                         bh.push(Reverse((forward_cost[next_node], next_node)));
//                     }
//                 }
//             }
//         }
//         // backword
//         bh.push(Reverse((0, i)));
//         let mut backword_cost = vec![MAX_COST; n];
//         backword_cost[i] = 0;
//         while let Some(Reverse(node)) = bh.pop() {
//             if let Some(next_nodes) = rev_edge.get_mut(&node.1) {
//                 next_nodes.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
//                 for next_node_cost in next_nodes {
//                     let next_node = next_node_cost.0;
//                     let next_cost = next_node_cost.1;
//                     let cost = backword_cost[node.1] + next_cost;
//                     if next_node != i && backword_cost[next_node] > cost {
//                         backword_cost[next_node] = cost;
//                         bh.push(Reverse((backword_cost[next_node], next_node)));
//                     }
//                 }
//             }
//         }
//         let mut min_cost = MAX_COST;
//         for j in 0..n {
//             if i == j {
//                 min_cost = min(min_cost, self_costs[i]);
//             } else {
//                 min_cost = min(min_cost, forward_cost[j] + backword_cost[j]);
//             }
//         }
//         if min_cost == MAX_COST {
//             println!("-1");
//         } else {
//             println!("{}", min_cost);
//         }
//     }
// }

// #[derive(Eq)]
// struct BhNode {
//     node_id: usize,
//     cost: i64,
// }
//
// impl Ord for BhNode {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.cost.cmp(&other.cost)
//     }
// }
// impl PartialOrd for BhNode {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cost.cmp(&other.cost))
//     }
// }
// impl PartialEq for BhNode {
//     fn eq(&self, other: &Self) -> bool {
//         self.cost.eq(&other.cost)
//     }
// }
//
// #[fastout]
// fn main() {
//     input! {
//         n: usize, m: usize,
//         abc: [(usize, usize, i64); m]
//     }
//     let mut self_costs = vec![MAX_COST; n];
//     let mut edge = HashMap::new();
//     let mut rev_edge = HashMap::new();
//     for (a_i, b_i, c_i) in abc {
//         (*edge.entry(a_i - 1).or_insert(vec![])).push((b_i - 1, c_i));
//         (*rev_edge.entry(b_i - 1).or_insert(vec![])).push((a_i - 1, c_i));
//         if a_i == b_i {
//             self_costs[a_i - 1] = min(self_costs[a_i - 1], c_i);
//         }
//     }
//     // dijkstra
//     for i in 0..n {
//         // forward
//         let mut bh = BinaryHeap::new();
//         bh.push(BhNode {
//             node_id: i,
//             cost: 0,
//         });
//         let mut forward_cost = vec![MAX_COST; n];
//         forward_cost[i] = 0;
//         while let Some(node) = bh.pop() {
//             if let Some(next_nodes) = edge.get_mut(&node.node_id) {
//                 next_nodes.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
//                 for next_node_cost in next_nodes {
//                     let next_node = next_node_cost.0;
//                     let next_cost = next_node_cost.1;
//                     let cost = forward_cost[node.node_id] + next_cost;
//                     if next_node != i && forward_cost[next_node] > cost {
//                         forward_cost[next_node] = cost;
//                         bh.push(BhNode {
//                             node_id: next_node,
//                             cost: forward_cost[next_node],
//                         });
//                     }
//                 }
//             }
//         }
//         // backword
//         bh.push(BhNode {
//             node_id: i,
//             cost: 0,
//         });
//         let mut backword_cost = vec![MAX_COST; n];
//         backword_cost[i] = 0;
//         while let Some(node) = bh.pop() {
//             if let Some(next_nodes) = rev_edge.get_mut(&node.node_id) {
//                 next_nodes.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
//                 for next_node_cost in next_nodes {
//                     let next_node = next_node_cost.0;
//                     let next_cost = next_node_cost.1;
//                     let cost = backword_cost[node.node_id] + next_cost;
//                     if next_node != i && backword_cost[next_node] > cost {
//                         backword_cost[next_node] = cost;
//                         bh.push(BhNode {
//                             node_id: next_node,
//                             cost: backword_cost[next_node],
//                         });
//                     }
//                 }
//             }
//         }
//         let mut min_cost = MAX_COST;
//         for j in 0..n {
//             if i == j {
//                 min_cost = min(min_cost, self_costs[i]);
//             } else {
//                 min_cost = min(min_cost, forward_cost[j] + backword_cost[j]);
//             }
//         }
//         if min_cost == MAX_COST {
//             println!("-1");
//         } else {
//             println!("{}", min_cost);
//         }
//     }
// }

// fn dfs(
//     first_flg: bool,
//     target: usize,
//     node: usize,
//     edge: &Vec<Vec<i64>>,
//     dp: &mut Vec<i64>,
// ) -> i64 {
//     let mut res = 1_000_000_000_000i64;
//     let self_value;
//     if first_flg {
//         self_value = 0;
//     } else {
//         if target == node {
//             return dp[node];
//         } else {
//             self_value = dp[node];
//         }
//     }
//     for j in 0..dp.len() {
//         if self_value + edge[node][j] < dp[j] {
//             dp[j] = self_value + edge[node][j];
//             res = min(res, dfs(false, target, j, edge, dp));
//         }
//     }
//     res
// }
//
// fn main() {
//     // priority queue 使った bfs とか？
//     input! {
//         n: usize, m: usize,
//         abc: [(usize, usize, i64); m]
//     }
//     let mut edge = vec![vec![1_000_000_000_000i64; n]; n];
//     for (a_i, b_i, c_i) in abc {
//         edge[a_i - 1][b_i - 1] = min(c_i, edge[a_i - 1][b_i - 1]);
//     }
//     for i in 0..n {
//         let mut dp = vec![1_000_000_000_000i64; n];
//         let res = dfs(true, i, i, &edge, &mut dp);
//         if res == 1_000_000_000_000i64 {
//             println!("-1");
//         } else {
//             println!("{}", res);
//         }
//     }
// }
