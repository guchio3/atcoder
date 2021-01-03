#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

// fn get_parents(
//     node: usize,
//     parents: &mut HashSet<usize>,
//     res_dict: &mut HashMap<usize, HashSet<usize>>,
//     link_dict: &HashMap<usize, Vec<usize>>,
//     accessed: &mut Vec<bool>,
// ) {
//     for &next_node in link_dict.get(&node).unwrap() {
//         if !accessed[next_node] {
//             accessed[next_node] = true;
//             parents.insert(next_node);
//             let mut tmp_parents = HashSet::new();
//             tmp_parents.extend(parents.iter());
//             res_dict.insert(next_node, tmp_parents);
//             get_parents(next_node, parents, res_dict, link_dict, accessed);
//             parents.remove(&next_node);
//         }
//     }
// }

fn get_depth(
    node: usize,
    depths: &mut Vec<usize>,
    link_dict: &HashMap<usize, Vec<usize>>,
    accessed: &mut Vec<bool>,
) {
    for &next_node in link_dict.get(&node).unwrap() {
        if !accessed[next_node] {
            accessed[next_node] = true;
            depths[next_node] = depths[node] + 1;
            get_depth(next_node, depths, link_dict, accessed);
        }
    }
}

// fn topological_sort(
//     node: usize,
//     res_vec: &mut Vec<usize>,
//     link_dict: &HashMap<usize, Vec<usize>>,
//     accessed: &mut Vec<bool>,
// ) {
//     for &next_node in link_dict.get(&node).unwrap() {
//         if !accessed[next_node] {
//             accessed[next_node] = true;
//             res_vec.push(next_node);
//             topological_sort(next_node, res_vec, link_dict, accessed);
//         }
//     }
// }

fn dfs(
    node: usize,
    res_vec: &mut Vec<i64>,
    cum_vec: &mut Vec<i64>,
    link_dict: &HashMap<usize, Vec<usize>>,
    accessed: &mut Vec<bool>,
) {
    res_vec[node] += cum_vec[node];
    for &next_node in link_dict.get(&node).unwrap() {
        if !accessed[next_node] {
            accessed[next_node] = true;
            cum_vec[next_node] += cum_vec[node];
            dfs(next_node, res_vec, cum_vec, link_dict, accessed);
        }
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
        q: usize,
        tex: [(usize, usize, i64); q]
    }
    let mut link_dict = HashMap::new();
    for &ab_i in ab.iter() {
        link_dict.entry(ab_i.0).or_insert(vec![]).push(ab_i.1);
        link_dict.entry(ab_i.1).or_insert(vec![]).push(ab_i.0);
    }

    // let mut accessed = vec![false; n + 1];
    // accessed[1] = true;
    // let mut res_dict = HashMap::new();
    // let tmp_set: HashSet<usize> = HashSet::new();
    // res_dict.insert(1, tmp_set);
    // let mut parents = HashSet::new();
    // parents.insert(1);
    // get_parents(1, &mut parents, &mut res_dict, &link_dict, &mut accessed);
    let mut accessed = vec![false; n + 1];
    accessed[1] = true;
    let mut depths = vec![0; n + 1];
    depths[1] = 1;
    get_depth(1, &mut depths, &link_dict, &mut accessed);

    let mut cum_vec: Vec<i64> = vec![0; n + 1];
    for tex_i in tex {
        let t = tex_i.0;
        let e = tex_i.1;
        let x = tex_i.2;
        let mut start = ab[e - 1].0;
        let mut end = ab[e - 1].1;
        if t == 2 {
            let tmp = start;
            start = end;
            end = tmp;
        }
        if depths[start] > depths[end] {
            cum_vec[start] += x;
        // if res_dict.get(&start).unwrap().contains(&end) {
        //     for node in link_dict.get(&end).unwrap() {
        //         if res_dict.get(&start).unwrap().contains(&node)
        //             && !res_dict.get(&end).unwrap().contains(&node)
        //         {
        //             // println!("tex: {:?}, node: {}", tex_i, node);
        //             cum_vec[*node as usize] += x;
        //             break;
        //         }
        //     }
        } else {
            cum_vec[1] += x;
            cum_vec[end] -= x;
        }
    }

    let mut accessed = vec![false; n + 1];
    accessed[1] = true;
    let mut res_vec = vec![0; n + 1];
    dfs(1, &mut res_vec, &mut cum_vec, &link_dict, &mut accessed);
    for i in 1..=n {
        println!("{}", res_vec[i]);
    }
}
