#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

// fn main() {
//     input! {
//         n: usize,
//         uvw: [(usize, usize, i64); n - 1]
//     }
//     let mut edges = HashMap::new();
//     for i in 0..n - 1 {
//         let (u, v, w) = uvw[i];
//         (*edges.entry(u - 1).or_insert(vec![])).push((v - 1, w));
//         (*edges.entry(v - 1).or_insert(vec![])).push((u - 1, w));
//     }
//
//     let mut accessed = vec![false; n];
//     let mut hop_num = vec![0; n];
//     let mut reses = vec![0; n];
//     let mut nodes = VecDeque::new();
//     nodes.push_back(0);
//     accessed[0] = true;
//     while nodes.len() > 0 {
//         let node = nodes.pop_front().unwrap();
//         if let Some(next_nodes) = edges.get(&node) {
//             for &(next_node, _next_w) in next_nodes {
//                 if !accessed[next_node] {
//                     nodes.push_back(next_node);
//                     reses[next_node] = hop_num[node] % 2;
//                     accessed[next_node] = true;
//                     hop_num[next_node] = hop_num[node] + 1;
//                 }
//             }
//         }
//     }
//     println!("{}", reses.into_iter().join("\n"));
// }

fn main() {
    input! {
        n: usize,
        uvw: [(usize, usize, i64); n - 1]
    }
    let mut edges = HashMap::new();
    for i in 0..n - 1 {
        let (u, v, w) = uvw[i];
        (*edges.entry(u - 1).or_insert(vec![])).push((v - 1, w));
        (*edges.entry(v - 1).or_insert(vec![])).push((u - 1, w));
    }

    let mut accessed = vec![false; n];
    let mut reses = vec![0; n];
    let mut nodes = VecDeque::new();
    nodes.push_back(0);
    accessed[0] = true;
    while nodes.len() > 0 {
        let node = nodes.pop_front().unwrap();
        if let Some(next_nodes) = edges.get(&node) {
            for &(next_node, next_w) in next_nodes {
                if !accessed[next_node] {
                    nodes.push_back(next_node);
                    if next_w % 2 == 0 {
                        reses[next_node] = reses[node];
                    } else {
                        reses[next_node] = (reses[node] + 1) % 2;
                    }
                    accessed[next_node] = true;
                }
            }
        }
    }
    println!("{}", reses.into_iter().join("\n"));
}
