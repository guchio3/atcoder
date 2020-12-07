struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut parents: Vec<usize> = vec![0; n];
        let sizes: Vec<usize> = vec![1; n];
        for i in 0..n {
            parents[i] = i;
        }
        return UnionFind { parents, sizes };
    }

    fn root(&mut self, x: usize) -> usize {
        // 経路圧縮を実装
        let parent = self.parents[x];
        if parent == x {
            x
        } else {
            let root = self.root(parent as usize);
            self.parents[x] = parent;
            root
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);

        if !self.isin_same_group(x, y) {
            if self.group_size_of(x) > self.group_size_of(y) {
                let temp_x = y;
                y = x;
                x = temp_x;
            }

            self.parents[x] = y;
            self.sizes[y] += self.sizes[x];
        }
    }

    fn _get_size_of(&mut self, root_x: usize) -> usize {
        if self.parents[root_x] != root_x {
            panic!("root_x should be root node.");
        }
        self.sizes[root_x]
    }

    fn group_size_of(&mut self, x: usize) -> usize {
        let root = self.root(x);
        return self._get_size_of(root);
    }

    fn isin_same_group(&mut self, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y);
    }

    fn max_group_size(&self) -> usize {
        let mut res = 0;
        for &size in self.sizes.iter() {
            res = max(res, size);
        }
        res
    }
}
