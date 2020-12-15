use num::traits::{NumAssign, PrimInt};
use std::fmt::Debug;

struct SegTree<T, F> {
    n: usize,
    nodes: Vec<T>,
    default_value: T,
    op: F,
}

impl<T, F> SegTree<T, F>
where
    T: PrimInt + NumAssign + Debug,
    F: Fn(T, T) -> T,
{
    fn new(num_nodes: usize, default_value: T, op: F) -> SegTree<T, F> {
        let n = 2usize.pow((num_nodes as f64).log2().ceil() as u32);
        let nodes = vec![default_value; n * 2 - 1];
        SegTree {
            n: n,
            nodes: nodes,
            default_value: default_value,
            op,
        }
    }

    fn init(&mut self, values: Vec<T>) {
        if self.n < values.len() {
            panic!(
                "too long values, self.n: {} -- values.len(): {}",
                self.n,
                values.len(),
            );
        }
        let mut i = self.n - 1;
        for value in values {
            self.nodes[i] = value;
            i += 1;
        }
        for j in (0..(self.n - 1)).rev() {
            self.nodes[j] = (self.op)(self.nodes[j * 2 + 1], self.nodes[j * 2 + 2]);
        }
    }

    fn update(&mut self, t: usize, v: T) {
        self._update(t, 0, v, 0, self.n)
    }

    fn _update(&mut self, t: usize, k: usize, v: T, l: usize, r: usize) {
        self.nodes[k] = (self.op)(self.nodes[k], v);
        if l + 1 < r {
            let half = (l + r) / 2;
            if t < half {
                self._update(t, k * 2 + 1, v, l, half);
            } else {
                self._update(t, k * 2 + 2, v, half, r);
            }
        }
    }

    fn query(&mut self, s: usize, e: usize) -> T {
        self._query(s, e, 0, 0, self.n)
    }

    fn _query(&mut self, s: usize, e: usize, k: usize, l: usize, r: usize) -> T {
        if r < s || e < l {
            self.default_value
        } else if s <= l && r <= e {
            self.nodes[k]
        } else {
            if l + 1 < r {
                let half = (l + r) / 2;
                let l_res = self._query(s, e, k * 2 + 1, l, half);
                let r_res = self._query(s, e, k * 2 + 2, half, r);
                (self.op)(l_res, r_res)
            } else {
                self.default_value
            }
        }
    }
}
