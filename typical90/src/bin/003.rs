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

fn calc_farthest_node(node: usize, edges: &HashMap<usize, Vec<usize>>) -> (usize, usize) {
    let mut deque = VecDeque::new();
    let mut used_nodes = HashSet::new();
    let mut farthest_node = node;
    let mut farthest_score = 0;

    deque.push_back((node, 0));
    used_nodes.insert(node);
    while deque.len() > 0 {
        let (node, score) = deque.pop_front().unwrap();
        for &next_node in edges.get(&node).unwrap().iter() {
            if !used_nodes.contains(&next_node) {
                if score + 1 > farthest_score {
                    farthest_score = score + 1;
                    farthest_node = next_node;
                }
                used_nodes.insert(next_node);
                deque.push_back((next_node, score + 1));
            }
        }
    }

    (farthest_node, farthest_score)
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    }
    // 木の直径を求める問題
    let mut edges = HashMap::new();
    for (a_i, b_i) in ab {
        edges.entry(a_i).or_insert(vec![]).push(b_i);
        edges.entry(b_i).or_insert(vec![]).push(a_i);
    }

    let (ank_node, _) = calc_farthest_node(1, &edges);
    let (_, res) = calc_farthest_node(ank_node, &edges);
    println!("{}", res + 1);
}
