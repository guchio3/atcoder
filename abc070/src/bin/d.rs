#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        abc: [(usize, usize, usize); n - 1],
        q: usize, k: usize,
        xy: [(usize, usize); q]
    }
    let mut edges = HashMap::new();
    for (a_i, b_i, c_i) in abc {
        (*edges.entry(a_i - 1).or_insert(vec![])).push((b_i - 1, c_i));
        (*edges.entry(b_i - 1).or_insert(vec![])).push((a_i - 1, c_i));
    }

    // dijkstra for k
    let mut bh = BinaryHeap::new();
    let mut used = vec![false; n];
    let mut distances_to_k = HashMap::new();
    used[k - 1] = true;
    distances_to_k.insert(k - 1, 0);
    bh.push((Reverse(0), k - 1));
    while bh.len() > 0 {
        let (Reverse(node_c), node) = bh.pop().unwrap();
        let next_node_info = edges.get(&node).unwrap();
        for &(next_node, next_node_c) in next_node_info {
            if !used[next_node] {
                used[next_node] = true;
                distances_to_k.insert(next_node, node_c + next_node_c);
                bh.push((Reverse(node_c + next_node_c), next_node));
            }
        }
    }

    for (x_i, y_i) in xy {
        let dist_to_x = distances_to_k.get(&(x_i - 1)).unwrap();
        let dist_to_y = distances_to_k.get(&(y_i - 1)).unwrap();
        println!("{}", dist_to_x + dist_to_y);
    }
}
