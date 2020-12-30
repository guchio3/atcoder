#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn rec_add_count(
    target: usize,
    count_vec: &mut Vec<usize>,
    tree_dict: &HashMap<usize, Vec<usize>>,
    accessed: &mut Vec<bool>,
) {
    if let Some(children) = tree_dict.get(&target) {
        for &child_i in children {
            if !accessed[child_i] {
                accessed[child_i] = true;
                count_vec[child_i] += count_vec[target];
                rec_add_count(child_i, count_vec, tree_dict, accessed);
            }
        }
    }
}

fn main() {
    input! {
        n: usize, q: usize,
        ab: [(usize, usize); n - 1],
        px: [(usize, usize); q]
    }
    let mut tree_dict = HashMap::new();
    for ab_i in ab {
        let a = ab_i.0;
        let b = ab_i.1;
        (*tree_dict.entry(a).or_insert(vec![])).push(b);
        (*tree_dict.entry(b).or_insert(vec![])).push(a);
    }

    let mut count_vec = vec![0; n + 1];
    for px_i in px {
        count_vec[px_i.0] += px_i.1;
    }

    let mut accessed = vec![false; n + 1];
    accessed[1] = true;
    rec_add_count(1, &mut count_vec, &tree_dict, &mut accessed);

    let res = count_vec[1..].iter().join(" ");
    println!("{}", res);
}
