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
    node: usize,
    edges: &HashMap<usize, Vec<usize>>,
    bef_color: usize,
    res_colors: &mut HashMap<(usize, usize), usize>,
    accessed: &mut Vec<bool>,
) {
    let next_nodes = edges.get(&node).unwrap();
    let mut color = 0;
    for &next_node in next_nodes {
        if !accessed[next_node] {
            color += 1;
            while color == bef_color {
                color += 1;
            }
            accessed[next_node] = true;
            res_colors.insert((node, next_node), color);
            res_colors.insert((next_node, node), color);
            dfs(next_node, edges, color, res_colors, accessed);
            accessed[next_node] = false;
        }
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    }
    let mut degs = vec![0; n];
    let mut edges = HashMap::new();
    for i in 0..n - 1 {
        let (a_i, b_i) = ab[i];
        degs[a_i - 1] += 1;
        degs[b_i - 1] += 1;
        (*edges.entry(a_i - 1).or_insert(vec![])).push(b_i - 1);
        (*edges.entry(b_i - 1).or_insert(vec![])).push(a_i - 1);
    }
    let k: usize = degs.into_iter().max().unwrap();
    println!("{}", k);

    let mut rest_colors = vec![];
    for i in (1..n).rev() {
        rest_colors.push(i);
    }
    let mut res_colors = HashMap::new();
    let mut accessed = vec![false; n];
    accessed[0] = true;
    dfs(0, &edges, 0, &mut res_colors, &mut accessed);
    for (a_i, b_i) in ab {
        println!("{}", res_colors.get(&(a_i - 1, b_i - 1)).unwrap());
    }

    // let k: usize = degs.iter().map(|x| *x).sum::<usize>();
    // println!("{}", k);

    // let mut invalid_colors = vec![vec![]; n - 1];
    // let mut degs = vec![0; n];
    // let mut reses = vec![];
    // for (a_i, b_i) in ab {
    //     degs[a_i - 1] += 1;
    //     degs[b_i - 1] += 1;
    //     println!("{:?}", degs);
    //     reses.push(max(degs[a_i - 1], degs[b_i - 1]));
    // }
    // let k: usize = degs.into_iter().max().unwrap();
    // println!("{}", k);
    // println!("{}", reses.into_iter().join("\n"));
}
