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
        n: usize, m: usize, x: usize, y: usize,
        abtk: [(usize, usize, usize, usize); m]
    }
    let mut edge = HashMap::new();
    for i in 0..m {
        let (a_i, b_i, t_i, k_i) = abtk[i];
        (*edge.entry(a_i - 1).or_insert(vec![])).push((b_i - 1, t_i, k_i));
        (*edge.entry(b_i - 1).or_insert(vec![])).push((a_i - 1, t_i, k_i));
    }
    let mut times = vec![10_000_000_000_000_000; n];
    let mut bh = BinaryHeap::new();
    times[x - 1] = 0;
    bh.push((0, x - 1));
    while let Some(node) = bh.pop() {
        if let Some(next_nodes) = edge.get(&node.1) {
            for next_node_pair in next_nodes {
                let next_node = next_node_pair.0;
                let next_cost = next_node_pair.1;
                let next_k = next_node_pair.2;
                let mut next_time = times[node.1] + next_cost;
                if (times[node.1] % next_k) != 0 {
                    next_time += next_k - times[node.1] % next_k;
                }
                if times[next_node] > next_time {
                    bh.push((next_time, next_node));
                    times[next_node] = next_time;
                }
            }
        }
    }
    if times[y - 1] == 10_000_000_000_000_000 {
        println!("-1");
    } else {
        println!("{}", times[y - 1]);
    }
}
