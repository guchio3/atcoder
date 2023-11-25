#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

// 二部グラフ判定
// 繋がりを greedy に追っていけば良い。
// 全部のグラフが繋がっていない可能性があるから、一応 DFS or BFS を全頂点を起点に開始
fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; m],
        b: [usize; m],
    }
    // map connections
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..m {
        graph[a[i] - 1].push(b[i] - 1);
        graph[b[i] - 1].push(a[i] - 1);
    }
    // iter
    let mut colors: Vec<i32> = vec![-1; n];
    for i in 0..n {
        if colors[i] != -1 {
            continue;
        }
        let mut bfs_queue = VecDeque::new();
        // init by 0 (both 0 or 1 are Ok)
        bfs_queue.push_back((i, 0));
        while bfs_queue.len() > 0 {
            let (node, color) = bfs_queue.pop_front().unwrap();
            if colors[node] == -1 {
                colors[node] = color;
                let next_color = (color + 1) % 2;
                for n in graph[node].iter() {
                    bfs_queue.push_back((*n, next_color));
                }
            } else {
                if color != colors[node] {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
