fn calc_farthest_node(node: usize, edges: &HashMap<usize, Vec<usize>>) -> (usize, usize) {
    // 重みなし無向グラフにおいて、ノードから最も遠いノードと、そのノードへの hop 数を返す
    let mut deque = VecDeque::new();
    let mut used_nodes = HashSet::new();
    let mut farthest_node = node;
    let mut farthest_score = 0;

    deque.push_back((node, 0));
    used_nodes.insert(node);
    while deque.len() > 0 {
        let (node, score) = deque.pop_front().unwrap();
        for &next_node in edges.get(&node).unwrap().iter() {
            if !used_nodes.contains(&next_node) {
                if score + 1 > farthest_score {
                    farthest_score = score + 1;
                    farthest_node = next_node;
                }
                used_nodes.insert(next_node);
                deque.push_back((next_node, score + 1));
            }
        }
    }

    (farthest_node, farthest_score)
}
