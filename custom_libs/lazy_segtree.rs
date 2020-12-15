use num::traits::{NumAssign, PrimInt};
use std::fmt::Debug;

struct LazySegTree<T> {
    n: usize,
    nodes: Vec<T>,
    lazy_nodes: Vec<T>,
}

impl<T> LazySegTree<T>
where
    T: PrimInt + NumAssign + Debug,
{
    fn new(n: usize) -> LazySegTree<T> {
        let nn = 2usize.pow((n as f64).log2() as u32 + 1);
        let nodes: Vec<T> = vec![T::zero(); 2 * n - 1];
        let lazy_nodes: Vec<T> = vec![T::zero(); 2 * n - 1];
        LazySegTree {
            n: nn,
            nodes: nodes,
            lazy_nodes: lazy_nodes,
        }
    }

    fn prop(&mut self, k: usize) {
        let v = self.lazy_nodes[k];
        // 更新するものがない
        if v != T::zero() {
            if k < self.n - 1 {
                self.lazy_nodes[k * 2 + 1] += v;
                self.lazy_nodes[k * 2 + 2] += v;
            }

            self.nodes[k] += self.lazy_nodes[k];
            self.lazy_nodes[k] = T::zero();
        }
    }

    fn update(&mut self, s: usize, e: usize, v: T) {
        self._update(s, e, v, 0, 0, self.n)
    }

    fn _update(&mut self, s: usize, e: usize, v: T, k: usize, l: usize, r: usize) {
        self.prop(k);
        if s <= l && r <= e {
            self.lazy_nodes[k] += v;
            self.prop(k);
        } else if s < r && l < e {
            self._update(s, e, v, k * 2 + 1, l, (l + r) / 2);
            self._update(s, e, v, k * 2 + 2, (l + r) / 2, r);
            self.nodes[k] = max(self.nodes[k * 2 + 1], self.nodes[k * 2 + 2]);
        }
    }

    fn query(&mut self, s: usize, e: usize) -> T {
        self._query(s, e, 0, 0, self.n)
    }

    fn _query(&mut self, s: usize, e: usize, k: usize, l: usize, r: usize) -> T {
        self.prop(k);
        if r <= s || e <= l {
            T::zero()
        } else if s <= l && r <= e {
            self.nodes[k]
        } else {
            max(
                self._query(s, e, k * 2 + 1, l, (l + r) / 2),
                self._query(s, e, k * 2 + 2, (l + r) / 2, r),
            )
        }
    }
}
