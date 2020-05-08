use std::collections::{HashMap, HashSet};
use std::io::stdin;

static D: i64 = 1_000_000_000 + 7;

fn unit_dfs_cnt_pattens(
    base_cnt: &mut i64,
    target_node: i32,
    link_map: &HashMap<i32, Vec<i32>>,
    used_nodes: &mut HashSet<i32>,
) -> i64 {
    let neighbor_nodes = &link_map[&target_node];
    used_nodes.insert(target_node);
    println!("{:?}", used_nodes);
    if link_map.len() == used_nodes.len() {
        *base_cnt += 1;
    }
    *base_cnt = *base_cnt % D;

    for neighbor_node in neighbor_nodes {
        if !used_nodes.contains(&neighbor_node) {
            unit_dfs_cnt_pattens(base_cnt, *neighbor_node, &link_map, used_nodes);
        }
    }

    used_nodes.remove(&target_node);
    *base_cnt
}

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).ok();
    let n: i32 = s.trim().parse().unwrap();
    s.clear();

    let mut edge_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for _i in 1..n {
        stdin().read_line(&mut s).ok();
        let tmp: Vec<i32> = s
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if edge_map.contains_key(&tmp[0]) {
            edge_map.get_mut(&tmp[0]).unwrap().push(tmp[1]);
        } else {
            edge_map.insert(tmp[0], vec![tmp[1]]);
        }
        if edge_map.contains_key(&tmp[1]) {
            edge_map.get_mut(&tmp[1]).unwrap().push(tmp[0]);
        } else {
            edge_map.insert(tmp[1], vec![tmp[0]]);
        }

        s.clear();
    }

    // implement logics
    let mut res: Vec<i64> = Vec::new();
    // for each k in 0..n
    for i in 1..n+1 {
        let mut k_res = 0;
        let mut used_nodes: HashSet<i32> = HashSet::new();
        k_res = unit_dfs_cnt_pattens(&mut k_res, i, &edge_map, &mut used_nodes);
        res.push(k_res);
        println!("{}", k_res);
    }

    for res_elem in res {
        println!("{}", res_elem);
    }
}
