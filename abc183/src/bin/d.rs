// ref. https://algo-logic.info/segment-tree/#:~:text=%E6%AF%94%E4%BE%8B%E3%81%97%E3%81%9F%E4%BD%9C%E7%94%A8%E7%B4%A0-,%E3%82%BB%E3%82%B0%E3%83%A1%E3%83%B3%E3%83%88%E6%9C%A8%E3%81%A8%E3%81%AF,%E9%A0%BB%E5%87%BA%E3%81%A8%E3%81%AA%E3%81%A3%E3%81%A6%E3%81%84%E3%81%BE%E3%81%99%E3%80%82
use num::traits::{PrimInt, NumAssign};
use proconio::{fastout, input};
use std::cmp::max;

// struct SegTree {
//     n: usize,
//     nodes: Vec<i64>,
// }
//
// impl SegTree {
//     fn new(n: usize) -> SegTree {
//         let nodes: Vec<i64> = vec![0; 2 * n - 1];
//         SegTree { n: n, nodes: nodes }
//     }
//
//     fn update(&mut self, mut i: usize, v: i64) {
//         i += self.n - 1;
//         self.nodes[i] += v;
//         while i > 0 {
//             i = (i - 1) / 2;
//             self.nodes[i] = max(self.nodes[i * 2 + 1], self.nodes[i * 2 + 2]);
//         }
//     }
//
//     fn query(&self, s: usize, e: usize) -> i64 {
//         self._query(s, e, 0, 0, self.n)
//     }
//
//     fn _query(&self, s: usize, e: usize, k: usize, l: usize, r: usize) -> i64 {
//         if r <= s || e <= l {
//             0
//         } else if s <= l && r <= e {
//             self.nodes[k]
//         } else {
//             max(
//                 self._query(s, e, k * 2 + 1, l, (l + r) / 2),
//                 self._query(s, e, k * 2 + 2, (l + r) / 2, r),
//             )
//         }
//     }
// }
//
// #[fastout]
// fn main() {
//     input! {
//         n: usize, w: i64,
//         stp: [(i64, i64, i64); n]
//     }
//     let mut st = SegTree::new(2usize.pow(200_001.0_f64.log2() as u32 + 1));
//     for (s_i, t_i, p_i) in stp {
//         for i in s_i..t_i {
//             st.update(i as usize, p_i);
//         }
//     }
//
//     if st.nodes[0] > w {
//         println!("No");
//     } else {
//         println!("Yes");
//     }
// }

// struct LazySegTree {
//     n: usize,
//     nodes: Vec<i64>,
//     lazy_nodes: Vec<i64>,
// }
//
// impl LazySegTree {
//     fn new(n: usize) -> LazySegTree {
//         let nn = 2usize.pow((n as f64).log2() as u32 + 1);
//         let nodes: Vec<i64> = vec![0; 2 * n - 1];
//         let lazy_nodes: Vec<i64> = vec![0; 2 * n - 1];
//         LazySegTree {
//             n: nn,
//             nodes: nodes,
//             lazy_nodes: lazy_nodes,
//         }
//     }
//
//     fn prop(&mut self, k: usize) {
//         let v = self.lazy_nodes[k];
//         // 更新するものがない
//         if v != 0 {
//             if k < self.n - 1 {
//                 self.lazy_nodes[k * 2 + 1] += v;
//                 self.lazy_nodes[k * 2 + 2] += v;
//             }
//
//             self.nodes[k] += self.lazy_nodes[k];
//             self.lazy_nodes[k] = 0;
//         }
//     }
//
//     fn update(&mut self, s: usize, e: usize, v: i64) {
//         self._update(s, e, v, 0, 0, self.n)
//     }
//
//     fn _update(&mut self, s: usize, e: usize, v: i64, k: usize, l: usize, r: usize) {
//         self.prop(k);
//         if s <= l && r <= e {
//             self.lazy_nodes[k] += v;
//             self.prop(k);
//         } else if s < r && l < e {
//             self._update(s, e, v, k * 2 + 1, l, (l + r) / 2);
//             self._update(s, e, v, k * 2 + 2, (l + r) / 2, r);
//             self.nodes[k] = max(self.nodes[k * 2 + 1], self.nodes[k * 2 + 2]);
//         }
//     }
//
//     fn query(&mut self, s: usize, e: usize) -> i64 {
//         self._query(s, e, 0, 0, self.n)
//     }
//
//     fn _query(&mut self, s: usize, e: usize, k: usize, l: usize, r: usize) -> i64 {
//         self.prop(k);
//         if r <= s || e <= l {
//             0
//         } else if s <= l && r <= e {
//             self.nodes[k]
//         } else {
//             max(
//                 self._query(s, e, k * 2 + 1, l, (l + r) / 2),
//                 self._query(s, e, k * 2 + 2, (l + r) / 2, r),
//             )
//         }
//     }
// }

struct LazySegTree<T> {
    n: usize,
    nodes: Vec<T>,
    lazy_nodes: Vec<T>,
}

impl<T> LazySegTree<T>
where
    T: PrimInt + NumAssign,
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

#[fastout]
fn main() {
    input! {
        n: usize, w: i64,
        stp: [(usize, usize, i64); n]
    }

    // 何故かこれで RE...
    let mut lst = LazySegTree::new(2_000_001);
    // let mut lst = LazySegTree::new(2_000_001);

    for (s_i, t_i, p_i) in stp {
        lst.update(s_i, t_i, p_i);
    }

    if lst.query(0, n) > w {
        println!("No");
    } else {
        println!("Yes");
    }
}

// fn main() {
//     input! {
//         n: usize, w: usize,
//         stp: [(usize, usize, usize); n]
//     }
//
//     let mut ss = vec![0; 200001];
//     let mut tt = vec![0; 200001];
//
//     for (s_i, t_i, p_i) in stp {
//         ss[s_i] += p_i;
//         tt[t_i] += p_i;
//     }
//
//     let mut tmp = 0;
//     for i in 0..200001 {
//         tmp += ss[i];
//         tmp -= tt[i];
//         if tmp > w {
//             println!("No");
//             return;
//         }
//     }
//
//     println!("Yes");
// }
