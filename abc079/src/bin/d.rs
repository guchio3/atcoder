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
    start: i64,
    i: i64,
    tmp_cost: i64,
    c: &Vec<Vec<i64>>,
    min_map: &mut HashMap<i64, i64>,
    accessed: &mut Vec<bool>,
) {
    let tmp_min_cost = min_map.get_mut(&start).unwrap();
    if tmp_cost > *tmp_min_cost {
    } else if i == 1 {
        *tmp_min_cost = tmp_cost;
    } else {
        for j in 0..10 {
            if !accessed[j as usize] {
                let add_cost = c[(i as usize)][j];
                accessed[j as usize] = true;
                dfs(start, j as i64, tmp_cost + add_cost, c, min_map, accessed);
                accessed[j as usize] = false;
            }
        }
    }
}

fn main() {
    input! {
        h: usize, w: usize,
        c: [[i64; 10]; 10],
        a: [[i64; w]; h]
    }
    let mut min_map = HashMap::new();
    for i in 0..10 {
        min_map.insert(i as i64, c[i][1]);
    }
    let mut accessed = vec![false; 10];
    for i in 0..10 {
        accessed[i] = true;
        dfs(i as i64, i as i64, 0, &c, &mut min_map, &mut accessed);
        accessed[i] = false;
    }

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            let aij = a[i][j];
            if aij != -1 {
                res += min_map.get(&aij).unwrap();
            }
        }
    }
    println!("{}", res);
}
