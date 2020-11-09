use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); k]
    }

    let mut uf = UnionFind::new(n + 1);
    let mut num_friends = vec![0; n + 1];
    for (a_i, b_i) in ab {
        uf.union(a_i, b_i);
        num_friends[a_i] += 1;
        num_friends[b_i] += 1;
    }

    let mut num_blocks = vec![0; n + 1];
    for (c_i, d_i) in cd {
        // 違う group だけど block している場合もある
        if uf.equiv(c_i, d_i) {
            num_blocks[c_i] += 1;
            num_blocks[d_i] += 1;
        }
    }

    let mut graph_size = vec![0; n + 1];
    for i in 1..=n {
        graph_size[uf.find(i)] += 1;
    }

    for i in 1..=n {
        let res = graph_size[uf.find(i)] - num_friends[i] - num_blocks[i] - 1;
        println!("{}", res);
    }
}
