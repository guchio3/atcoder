#![allow(unused_imports)]
use petgraph::unionfind::UnionFind;
use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize, m: usize,
        uvc: [(usize, usize, usize); m]
    }
    let mut t = vec![vec![]; n];
    // let mut used: HashSet<usize> = HashSet::new();
    let mut uf = UnionFind::new(n);

    // for i in 0..n {
    //     used.insert(i);
    // }

    for &(u_i, v_i, c_i) in &uvc {
        t[v_i - 1].push((u_i - 1, c_i));
        t[u_i - 1].push((v_i - 1, c_i));
    }

    let mut res = vec![0; n];
    res[0] = 1;
    let mut cand = vec![0];
    while let Some(l) = cand.pop() {
        while let Some((node, c)) = t[l].pop() {
            if uf.find(l) != uf.find(node) {
                // println!(
                //     "n: {}, l: {}, c: {}, res[l]: {}, (c+1)%n+1: {}",
                //     n,
                //     l,
                //     c,
                //     res[l],
                //     (c + 1) % n + 1
                // );
                cand.push(node);
                uf.union(l, node);
                res[node] = if res[l] == c { (c + 1) % n + 1 } else { c };
            }
        }
    }

    for res_i in res {
        println!("{}", res_i);
    }
}
