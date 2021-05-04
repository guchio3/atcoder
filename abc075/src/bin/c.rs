use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m]
    }
    let mut edges = vec![vec![false; n]; n];
    for &(ai, bi) in ab.iter() {
        edges[ai - 1][bi - 1] = true;
        edges[bi - 1][ai - 1] = true;
    }
    let mut res = 0;
    for i in 0..m {
        let invalid_edge = ab[i];
        let mut deque = VecDeque::new();
        let mut accessed = vec![false; n];
        deque.push_back(0);
        accessed[0] = true;
        while deque.len() > 0 {
            let target_node = deque.pop_front().unwrap();
            for j in 0..n {
                if edges[target_node][j]
                    && !accessed[j]
                    && !((target_node == invalid_edge.0 - 1 && j == invalid_edge.1 - 1)
                        || (target_node == invalid_edge.1 - 1 && j == invalid_edge.0 - 1))
                {
                    deque.push_back(j);
                    accessed[j] = true;
                }
            }
        }
        let accessed_num = accessed.into_iter().fold(0, |x, a| x + a as usize);
        if accessed_num != n {
            res += 1;
        }
    }
    println!("{}", res);
}
