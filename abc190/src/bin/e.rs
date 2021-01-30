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
    target: usize,
    shortest_paths: &HashMap<usize, HashMap<petgraph::prelude::NodeIndex, usize>>,
    target_vec: Vec<usize>,
    used_vec: Vec<usize>,
    tmp_cost: i64,
) -> i64 {
    if target_vec.len() == 0 {
        return tmp_cost;
    }
    let mut tmp_res = 1_999_999_999_999;
    if !shortest_paths.contains_key(&target) {
        return tmp_res;
    } else {
        for &target_vec_i in target_vec.iter() {
            if used_vec.contains(&target_vec_i) {
                continue;
            }
            let next_target_vec: Vec<usize> = target_vec
                .iter()
                .map(|x| *x)
                .filter(|x| *x != target_vec_i)
                .collect();
            let mut next_used_vec: Vec<usize> = used_vec.iter().map(|x| *x).collect();
            next_used_vec.push(target_vec_i);
            let mut next_tmp_cost = tmp_cost;
            if let Some(edge_hasp_map) = shortest_paths.get(&target) {
                if let Some(cost) =
                    edge_hasp_map.get(&petgraph::graph::NodeIndex::new(target_vec_i))
                {
                    next_tmp_cost += *cost as i64;
                } else {
                    continue;
                }
            } else {
                continue;
            }
            tmp_res = min(
                tmp_res,
                dfs(
                    target_vec_i,
                    &shortest_paths,
                    next_target_vec,
                    next_used_vec,
                    next_tmp_cost,
                ),
            );
        }
        if tmp_res != 1_999_999_999_999 {
            tmp_res
        } else {
            -1
        }
    }
}

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
        k: usize,
        c: [usize; k]
    }

    let mut graph = petgraph::Graph::<usize, usize>::new();
    for i in 0..n {
        graph.add_node(i);
    }
    for (a_i, b_i) in ab {
        graph.add_edge(
            petgraph::graph::NodeIndex::new(a_i - 1),
            petgraph::graph::NodeIndex::new(b_i - 1),
            1,
        );
        graph.add_edge(
            petgraph::graph::NodeIndex::new(b_i - 1),
            petgraph::graph::NodeIndex::new(a_i - 1),
            1,
        );
    }
    let mut shortest_paths = HashMap::new();
    for &c_i in c.iter() {
        let res = petgraph::algo::dijkstra(
            &graph,
            petgraph::prelude::NodeIndex::new(c_i - 1),
            None,
            |e| *e.weight(),
        );
        shortest_paths.insert(c_i, res);
    }
    let c: Vec<usize> = c.into_iter().map(|x| x - 1).collect();
    let mut res = 1_999_999_999_999;
    for &c_i in c.iter() {
        let tmp_c = c.iter().map(|x| *x).filter(|x| *x != c_i).collect();
        let tmp_res = dfs(c_i, &shortest_paths, tmp_c, vec![], 0);
        if tmp_res > 0 {
            res = min(res, tmp_res);
        }
    }
    if res == 1_999_999_999_999 {
        res = -1;
    }
    println!("{}", res);
}
