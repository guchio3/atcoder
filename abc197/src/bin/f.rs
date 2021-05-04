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
        abc: [(usize, usize, usize); m]
    }
    let mut edges = vec![vec![]; n + 1];
    for (a_i, b_i, c_i) in abc {
        edges[a_i].push((b_i, c_i));
        edges[b_i].push((a_i, c_i));
    }
    // 1 & n から N hop で行ける頂点を出していき、
    // 都度共通文字列があるかを判定
    // 共通文字列がないなら終了
    // BFS の訪問終了が無い ver ?
    let mut next_nodes = vec![(1, "")];
    let mut back_next_nodes = vec![(n, "")];
    while next_nodes.len() > 0 && back_next_nodes.len() > 0 {
        let mut tmp_next_nodes = vec![];
        let mut next_strings = HashSet::new();
        let mut tmp_back_next_nodes = vec![];
        let mut back_next_strings = HashSet::new();
        for &(next_node, next_string) in next_nodes.iter() {
            for &(next_next_node, next_next_string) in edges[next_node].iter() {
                tmp_next_nodes.push((
                    next_next_node,
                    format!("{}{}", next_string, next_next_string),
                ));
                next_strings.insert(format!("{}{}", next_string, next_next_string));
            }
        }
        for &(back_next_node, back_next_string) in back_next_nodes.iter() {
            for &(back_next_next_node, back_next_next_string) in edges[back_next_node].iter() {
                if next_strings.contains(&format!("{}{}", back_next_next_string, back_next_string))
                {
                    tmp_back_next_nodes.push((
                        back_next_next_node,
                        &format!("{}{}", back_next_next_string, back_next_string),
                    ));
                    back_next_strings
                        .insert(&format!("{}{}", back_next_next_string, back_next_string));
                }
            }
        }
        tmp_next_nodes = tmp_next_nodes
            .into_iter()
            .filter(|x| back_next_strings.contains(&x.1))
            .collect();
    }
    println!("");
}
