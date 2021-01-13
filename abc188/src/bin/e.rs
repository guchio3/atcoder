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
    town: usize,
    a: &Vec<i64>,
    mut chepest_map: &mut Vec<i64>,
    edge_dict: &HashMap<usize, Vec<usize>>,
    mut cheapest: i64,
    mut best_earn: i64,
    is_first: bool,
) -> i64 {
    if !is_first {
        best_earn = max(best_earn, a[town - 1] - cheapest);
    }
    cheapest = min(cheapest, a[town - 1]);
    if cheapest > chepest_map[town] {
        return best_earn;
    }
    if let Some(next_towns) = edge_dict.get(&town) {
        for next_town in next_towns {
            best_earn = max(
                best_earn,
                dfs(
                    *next_town,
                    &a,
                    &mut chepest_map,
                    &edge_dict,
                    cheapest,
                    best_earn,
                    false,
                ),
            );
        }
    }
    best_earn
}

fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; n],
        mut xy: [(usize, usize); m]
    }
    xy.sort_by(|xx, yy| xx.1.partial_cmp(&yy.1).unwrap());
    let mut edge_dict = HashMap::new();
    let mut is_source = vec![true; n + 1];
    let mut chepest_map = vec![10_000_000_000; n + 1];
    for (x_i, y_i) in xy {
        (*edge_dict.entry(x_i).or_insert(vec![])).push(y_i);
        is_source[y_i] = false;
        chepest_map[y_i] = min(chepest_map[y_i], a[x_i - 1]);
        chepest_map[y_i] = min(chepest_map[y_i], chepest_map[x_i]);
    }

    let mut res = a[0] - chepest_map[1];
    // for i in 1..=n {
    //     if is_source[i] {
    //         res = max(res, dfs(i, &a, &mut chepest_map, &edge_dict, a[i - 1], -1_000_000_000 * 2, true));
    //     }
    // }
    for i in 2..=n {
        res = max(res, a[i - 1] - chepest_map[i]);
    }
    println!("{}", res);
}
