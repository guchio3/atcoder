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
    node: usize,
    c: &Vec<usize>,
    edges: &HashMap<usize, Vec<usize>>,
    reses: &mut Vec<usize>,
    used_colors: &mut Vec<bool>,
    accessed: &mut Vec<bool>,
) {
    let next_nodes = edges.get(&node).unwrap();
    for &next_node in next_nodes {
        if !accessed[next_node] {
            let mut now_used = false;
            if !used_colors[c[next_node]] {
                reses.push(next_node + 1);
                used_colors[c[next_node]] = true;
                now_used = true;
            }
            accessed[node] = true;
            dfs(next_node, c, edges, reses, used_colors, accessed);
            if now_used {
                used_colors[c[next_node]] = false;
            }
            // accessed[next_node] = false;
        }
    }
}

fn main() {
    input! {
        n: usize,
        c: [usize; n],
        ab: [(usize, usize); n - 1]
    }
    let mut edges = HashMap::new();
    for (a_i, b_i) in ab {
        (*edges.entry(&a_i - 1).or_insert(vec![])).push(b_i - 1);
        (*edges.entry(&b_i - 1).or_insert(vec![])).push(a_i - 1);
    }

    // let mut bh = BinaryHeap::new();
    // let mut accessed = vec![false; n];

    // bh.push((Reverse));
    // while bh.len() > 0 {
    //     let node = bh
    // }
    // let mut nodes = VecDeque::new();
    // let mut used_colors: Vec<HashSet<usize>> = vec![];
    let mut accessed = vec![false; n];
    // let mut used_colors: Vec<HashSet<usize>> = vec![];
    let mut used_colors: Vec<bool> = vec![false; 1_000_000];
    let mut reses = vec![1];
    accessed[0] = true;
    used_colors[c[0]] = true;
    dfs(0, &c, &edges, &mut reses, &mut used_colors, &mut accessed);
    // for _i in 0..n {
    //     let mut new_set = HashSet::new();
    //     new_set.insert(c[0]);
    //     used_colors.push(new_set);
    // }
    // let mut reses = vec![1];
    // accessed[0] = true;
    // nodes.push_back(0);
    // while nodes.len() > 0 {
    //     let node = nodes.pop_front().unwrap();
    //     // println!("node: {}", node);
    //     // println!("used_colors: {:?}", used_colors);
    //     let next_nodes = edges.get(&node).unwrap();
    //     for &next_node in next_nodes {
    //         if !accessed[next_node] {
    //             if !used_colors[node].contains(&c[next_node]) {
    //                 reses.push(next_node + 1);
    //             }
    //             used_colors[next_node] = used_colors[node].clone();
    //             used_colors[next_node].insert(c[next_node]);
    //             accessed[node] = true;
    //             nodes.push_back(next_node);
    //         }
    //     }
    // }
    reses.sort();
    println!("{}", reses.into_iter().join("\n"));
}
