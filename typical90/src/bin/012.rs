#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use ordered_float::NotNan;
use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn to_index(r_i: usize, c_i: usize, w: usize) -> usize {
    w * r_i + c_i
}

fn main() {
    input! {
        h: usize, w: usize,
        q: usize,
    }
    let mut colors = vec![vec!['w'; w]; h];
    let mut uf = UnionFind::new((h + 1) * (w + 1));

    for _ in 0..q {
        input! {
            t_i: usize
        }
        if t_i == 1 {
            input! {
                r_i: usize, c_i: usize
            }
            colors[r_i - 1][c_i - 1] = 'r';
            if r_i > 1 && colors[r_i - 1 - 1][c_i - 1] == 'r' {
                uf.union(to_index(r_i, c_i, w), to_index(r_i - 1, c_i, w));
            }
            if r_i < h && colors[r_i - 1 + 1][c_i - 1] == 'r' {
                uf.union(to_index(r_i, c_i, w), to_index(r_i + 1, c_i, w));
            }
            if c_i > 1 && colors[r_i - 1][c_i - 1 - 1] == 'r' {
                uf.union(to_index(r_i, c_i, w), to_index(r_i, c_i - 1, w));
            }
            if c_i < w && colors[r_i - 1][c_i - 1 + 1] == 'r' {
                uf.union(to_index(r_i, c_i, w), to_index(r_i, c_i + 1, w));
            }
        } else {
            input! {
                ra_i: usize, ca_i: usize, rb_i: usize, cb_i: usize
            }
            if colors[ra_i - 1][ca_i - 1] == 'r'
                && colors[rb_i - 1][cb_i - 1] == 'r'
                && uf.find(to_index(ra_i, ca_i, w)) == uf.find(to_index(rb_i, cb_i, w))
            {
                println!("Yes");
            } else {
                println!("No");
            }
        }
        // println!("{:?}", uf);
    }
}
